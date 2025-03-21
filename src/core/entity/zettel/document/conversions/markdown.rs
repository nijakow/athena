use crate::core::entity;
use crate::core::entity::zettel::document;
use crate::formats::markdown;

pub type ConversionError = ();

fn convert_node(node: &markdown::Node) -> Result<document::node::Node, ConversionError> {
    fn styled(
        nodes: &markdown::Nodes,
        style: document::node::Style,
    ) -> Result<document::node::Node, ConversionError> {
        Ok(document::node::Node::Styled(
            style,
            Box::new(document::node::Node::Grouped(convert_nodes(nodes)?)),
        ))
    }

    match node {
        markdown::Node::Newline => Ok(document::node::Node::Newline),
        markdown::Node::Text(text) => Ok(document::node::Node::Text(text.clone())),
        markdown::Node::Code(code) => Ok(document::node::Node::Code(code.clone())),
        markdown::Node::Bold(nodes) => styled(nodes, document::node::Style::Bold),
        markdown::Node::Italic(nodes) => styled(nodes, document::node::Style::Italic),
        markdown::Node::Tag(tag) => Ok(document::node::Node::Tag(tag.clone())),
        markdown::Node::Link { embed, link } => {
            let target = &link.target;

            let result = match target {
                markdown::LinkTarget::Zettel(zettel) => Some(document::node::Node::Reference(document::node::Reference {
                    target: entity::Id::from_string(zettel),
                    caption: match &link.title {
                        Some(title) => convert_nodes(&title)?,
                        None => vec![document::node::Node::Text(zettel.clone())],
                    },
                    embed: *embed,
                })),
                markdown::LinkTarget::Url(_url) => None,
                markdown::LinkTarget::FreeForm(_) => None,
            };

            match result {
                Some(node) => Ok(node),
                None => Ok(document::node::Node::Text("TODO".to_string())),
            }
        }
        _ => Ok(document::node::Node::Text("TODO".to_string())),
    }
}

fn convert_nodes(nodes: &markdown::Nodes) -> Result<document::Nodes, ConversionError> {
    nodes.iter().map(convert_node).collect()
}

fn convert_heading(
    heading: &markdown::Heading,
) -> Result<document::block::Heading, ConversionError> {
    Ok(document::block::Heading::new(
        heading.0,
        convert_nodes(&heading.1)?,
    ))
}

fn convert_code(
    language: &Option<String>,
    code: &String,
) -> Result<document::block::CodeBlock, ConversionError> {
    Ok(document::block::CodeBlock {
        language: language.clone(),
        code: code.clone(),
    })
}

fn convert_block(block: &markdown::Block) -> Result<document::block::Block, ConversionError> {
    match block {
        markdown::Block::ThematicBreak => Ok(document::block::Block::Line),
        markdown::Block::Heading(heading) => {
            convert_heading(heading).map(document::block::Block::Heading)
        }
        markdown::Block::Code(lang, code) => {
            convert_code(lang, code).map(document::block::Block::CodeBlock)
        }
        markdown::Block::Callout(kind, callout) => Ok(document::block::Block::Callout(
            document::block::callout::Callout::new(
                match kind {
                    Some(kind) => document::block::callout::Kind::from(kind.as_str()),
                    None => document::block::callout::Kind::Basic,
                },
                convert_blocks(callout)?,
            ),
        )),
        markdown::Block::Nodes(nodes) => Ok(document::block::Block::Paragraph(
            document::block::Paragraph {
                nodes: convert_nodes(nodes)?,
            },
        )),
        markdown::Block::BulletPoint(bullet_point) => {
            // TODO
            Ok(document::block::Block::Paragraph(
                document::block::Paragraph {
                    nodes: convert_nodes(&bullet_point.1)?,
                },
            ))
        }
    }
}

fn convert_blocks(blocks: &markdown::Blocks) -> Result<document::Blocks, ConversionError> {
    blocks.iter().map(convert_block).collect()
}

pub fn markdown_to_document(
    markdown: &markdown::Document,
) -> Result<document::Document, ConversionError> {
    Ok(document::Document::with_blocks(convert_blocks(
        &markdown.blocks,
    )?))
}
