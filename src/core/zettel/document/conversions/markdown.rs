use crate::core::zettel::document;
use crate::formats::markdown;

pub type ConversionError = ();

fn convert_span(span: &markdown::model::Span) -> Result<document::node::Node, ()> {
    match span {
        markdown::model::Span::Break => Ok(document::node::Node::Newline),
        markdown::model::Span::Text(text) => Ok(document::node::Node::Text(text.clone())),
        markdown::model::Span::Code(code) => Ok(document::node::Node::Text(code.clone())),
        markdown::model::Span::Link(_, _, _) => todo!(),
        markdown::model::Span::Image(_, _, _) => todo!(),
        markdown::model::Span::Emphasis(spans) => Ok(document::node::Node::Styled(document::node::Style::Italic, Box::new(document::node::Node::Grouped(convert_spans(spans)?)))),
        markdown::model::Span::Strong(spans) => Ok(document::node::Node::Styled(document::node::Style::Bold, Box::new(document::node::Node::Grouped(convert_spans(spans)?)))),
        _ => Ok(document::node::Node::Text("Whoopsie!".to_string())),
    }
}

fn convert_spans(spans: &markdown::model::Spans) -> Result<document::Nodes, ()> {
    spans.iter().map(convert_span).collect()
}

fn convert_header(
    spans: &markdown::model::Spans,
    index: usize,
) -> Result<document::block::Block, ()> {
    Ok(document::block::Block::Heading(
        document::block::Heading::new(index as u8, convert_spans(spans)?),
    ))
}

fn convert_paragraph(spans: &markdown::model::Spans) -> Result<document::block::Block, ()> {
    Ok(document::block::Block::Paragraph(
        document::block::Paragraph {
            nodes: convert_spans(spans)?,
        },
    ))
}

fn convert_blockquote(
    kind: &Option<String>,
    nodes: &markdown::model::Nodes,
) -> Result<document::block::Block, ()> {
    Ok(document::block::Block::Callout(
        document::block::callout::Callout {
            kind: match kind {
                Some(kind) => kind.as_str().into(),
                None => document::block::callout::Kind::Basic,
            },
            blocks: convert_toplevels(nodes)?,
        },
    ))
}

fn convert_codeblock(language: &Option<String>, text: &str) -> Result<document::block::Block, ()> {
    Ok(document::block::Block::CodeBlock(
        document::block::CodeBlock {
            language: language.clone(),
            code: text.to_string(),
        },
    ))
}

fn convert_raw(text: &str) -> Result<document::block::Block, ()> {
    Ok(document::block::Block::Paragraph(
        document::block::Paragraph {
            nodes: vec![document::node::Node::Text(text.to_string())],
        },
    ))
}

fn convert_hr() -> Result<document::block::Block, ()> {
    Ok(document::block::Block::Line)
}

fn convert_toplevel(toplevel: &markdown::model::Node) -> Result<document::block::Block, ()> {
    match toplevel {
        markdown::model::Node::Header(spans, index) => convert_header(spans, *index),
        markdown::model::Node::Paragraph(spans) => convert_paragraph(spans),
        markdown::model::Node::Blockquote(language, spans) => convert_blockquote(language, spans),
        markdown::model::Node::CodeBlock(language, text) => convert_codeblock(language, text),
        markdown::model::Node::Raw(text) => convert_raw(text),
        markdown::model::Node::Hr => convert_hr(),
        _ => Err(()),
    }
}

fn convert_toplevels(toplevels: &markdown::model::Nodes) -> Result<document::Blocks, ()> {
    toplevels.iter().map(convert_toplevel).collect()
}

pub fn markdown_to_document(
    markdown: &markdown::Document,
) -> Result<document::Document, ConversionError> {
    Ok(document::Document::with_blocks(convert_toplevels(
        &markdown.toplevels,
    )?))
}
