use crate::{core::semantic::tag, formats::markdown::model as our};

use markdown as foreign;
use regex;

pub struct ParseContext<'a> {
    tags: &'a mut tag::Tags,
}

impl<'a> ParseContext<'a> {
    pub fn new(tags: &'a mut tag::Tags) -> ParseContext<'a> {
        ParseContext { tags }
    }

    pub fn get_or_add_tag(&mut self, name: &str) -> our::Tag {
        self.tags.get_or_add_tag(name)
    }
}


#[derive(Debug, Clone)]
enum TextAtom {
    Text(String),
    Tag(our::Tag),
    Wikilink(our::Wikilink),
    Label(String),
}

fn parse_wikilink(wikilink: &str, embedded: bool) -> our::Wikilink {
    let parts: Vec<&str> = wikilink.split('|').collect();

    let target = parts[0].to_string();
    let label = if parts.len() > 1 {
        Some(parts[1].to_string())
    } else {
        None
    };

    our::Wikilink {
        target,
        label,
        embedded,
    }
}

fn split_text_into_atoms(text: &str, context: &mut ParseContext) -> Vec<TextAtom> {
    let tag_regex = regex::Regex::new(r"#([a-zA-Z0-9]+(?:-[a-zA-Z0-9]+)*)").unwrap();
    let embedded_wikilink_regex = regex::Regex::new(r"!\[\[([^\]]+)\]\]").unwrap();
    let wikilink_regex = regex::Regex::new(r"\[\[([^\]]+)\]\]").unwrap();
    let label_regex = regex::Regex::new(r"\^([a-zA-Z0-9]+(?:-[a-zA-Z0-9]+)*)").unwrap();

    let combined_regex = regex::Regex::new(&format!(
        "{}|{}|{}|{}",
        tag_regex, embedded_wikilink_regex, wikilink_regex, label_regex
    ))
    .unwrap();

    let mut atoms = vec![];

    let mut last_end = 0;

    for cap in combined_regex.captures_iter(text) {
        let match_start = cap.get(0).unwrap().start();
        let match_end = cap.get(0).unwrap().end();

        // Add the text before the tag if there's any
        if match_start > last_end {
            atoms.push(TextAtom::Text(text[last_end..match_start].to_string()));
        }

        // Check which part of the capture matched and push the corresponding token
        if let Some(tag) = cap.get(1) {
            atoms.push(TextAtom::Tag(context.get_or_add_tag(tag.as_str())));
        } else if let Some(embedded_wikilink) = cap.get(2) {
            atoms.push(TextAtom::Wikilink(parse_wikilink(
                embedded_wikilink.as_str(),
                true,
            )));
        } else if let Some(wikilink) = cap.get(3) {
            atoms.push(TextAtom::Wikilink(parse_wikilink(wikilink.as_str(), false)));
        } else if let Some(label) = cap.get(4) {
            atoms.push(TextAtom::Label(label.as_str().to_string()));
        }

        // Update the last end position
        last_end = match_end;
    }

    if last_end < text.len() {
        atoms.push(TextAtom::Text(text[last_end..].to_string()));
    }

    atoms
}

fn atom_as_span(atom: TextAtom) -> our::Span {
    match atom {
        TextAtom::Text(text) => our::Span::Text(text),
        TextAtom::Tag(tag) => our::Span::Tag(tag),
        TextAtom::Wikilink(wikilink) => our::Span::Wikilink(wikilink),
        TextAtom::Label(label) => our::Span::Label(label),
    }
}

pub fn from_markdown_span(span: &foreign::Span, context: &mut ParseContext) -> Vec<our::Span> {
    match span {
        foreign::Span::Text(text) => split_text_into_atoms(text, context)
            .into_iter()
            .map(atom_as_span)
            .collect(),
        foreign::Span::Code(code) => vec![our::Span::Code(code.clone())],
        foreign::Span::Link(url, title, alt) => {
            vec![our::Span::Link(url.clone(), title.clone(), alt.clone())]
        }
        foreign::Span::Image(url, title, alt) => {
            vec![our::Span::Image(url.clone(), title.clone(), alt.clone())]
        }
        foreign::Span::Emphasis(spans) => {
            vec![our::Span::Emphasis(from_markdown_spans(spans, context))]
        }
        foreign::Span::Strong(spans) => vec![our::Span::Strong(from_markdown_spans(spans, context))],
        _ => vec![our::Span::Unknown],
    }
}

pub fn from_markdown_spans(spans: &Vec<foreign::Span>, context: &mut ParseContext) -> Vec<our::Span> {
    spans.iter().map(|span| from_markdown_span(span, context)).flatten().collect()
}

pub fn from_markdown_list_item(item: &foreign::ListItem, context: &mut ParseContext) -> our::ListItem {
    match item {
        foreign::ListItem::Simple(spans) => our::ListItem::Simple(from_markdown_spans(spans, context)),
        foreign::ListItem::Paragraph(spans) => {
            our::ListItem::Paragraph(from_markdown_blocks(spans, context))
        }
    }
}

pub fn from_markdown_block(block: &foreign::Block, context: &mut ParseContext) -> our::Node {
    match block {
        foreign::Block::Header(spans, level) => {
            our::Node::Header(from_markdown_spans(spans, context), *level)
        }
        foreign::Block::Paragraph(spans) => our::Node::Paragraph(from_markdown_spans(spans, context)),
        foreign::Block::Blockquote(blocks) => {
            our::Node::Blockquote(None, from_markdown_blocks(blocks, context))
        }
        foreign::Block::CodeBlock(lang, code) => our::Node::CodeBlock(lang.clone(), code.clone()),
        foreign::Block::Raw(text) => our::Node::Raw(text.clone()),
        foreign::Block::OrderedList(items, _) => our::Node::OrderedList(
            items.iter().map(|item| from_markdown_list_item(item, context)).collect(),
            our::OrderedListType::Undefined,
        ),
        foreign::Block::UnorderedList(items) => {
            our::Node::UnorderedList(items.iter().map(|item| from_markdown_list_item(item, context)).collect())
        }
        foreign::Block::Hr => our::Node::Hr,
    }
}

pub fn from_markdown_blocks(blocks: &Vec<foreign::Block>, context: &mut ParseContext) -> our::Nodes {
    blocks
        .iter()
        .map(|block| from_markdown_block(block, context))
        .collect()
}

pub fn from_markdown_document(properties: Option<yaml_rust::Yaml>, blocks: &Vec<foreign::Block>, context: &mut ParseContext) -> our::Document {
    our::Document {
        properties,
        toplevels: from_markdown_blocks(blocks, context),
    }
}

#[derive(Debug)]
pub enum ParseError {
    Unknown,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Unknown parse error")
    }
}

impl std::error::Error for ParseError {}

pub fn parse_document<S: ToString>(input: S, context: ParseContext) -> Result<our::Document, ParseError> {
    let mut context = context;

    // Extract the properties of the markdown file.
    //
    // Properties look like this:
    //
    // ```markdown
    // ---
    // key: value
    // key2: value2
    // ---
    //
    // Rest of the document...
    // ```
    //

    fn extract_property_string(input: String) -> (Option<String>, String) {
        // Extract the first line of the input by finding the first newline character. DON'T use input.lines() here,
        // because the file might be large.

        if let Some(first_newline) = input.find('\n') {
            let first_line = &input[..first_newline];

            // If the first line is "---", we have a property string.
            if first_line.trim() == "---" {
                // Find the second "---".
                if let Some(end_of_properties) = input[first_newline + 1..].find("\n---\n") {
                    let properties = &input[first_newline + 1..first_newline + 1 + end_of_properties];
                    let body = &input[first_newline + 1 + end_of_properties + 5..];

                    return (Some(properties.to_string()), body.to_string());
                }
            }
        }

        (None, input)
    }

    let (properties, body) = extract_property_string(input.to_string());

    let parsed_properties = properties.map(|properties| {
        yaml_rust::YamlLoader::load_from_str(&properties)
            .map(|mut docs| docs.pop().unwrap())
            .unwrap_or(yaml_rust::Yaml::BadValue)
    });

    let blocks = foreign::tokenize(&body);

    Ok(from_markdown_document(parsed_properties, &blocks, &mut context))
}
