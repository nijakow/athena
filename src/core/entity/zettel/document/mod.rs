
pub mod conversions;

pub mod block;
pub mod node;


pub type Blocks = Vec<block::Block>;
pub type Nodes = Vec<node::Node>;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Document {
    blocks: Blocks,
}

impl Document {
    pub fn test() -> Self {
        let paragraph = block::Paragraph {
            nodes: vec![
                node::Node::Text("Hello, ".to_string()),
                node::Node::Styled(
                    node::Style::Bold,
                    Box::new(node::Node::Text("world".to_string())),
                ),
                node::Node::Text("!".to_string()),
            ],
        };

        let paragraph2 = block::Paragraph {
            nodes: vec![
                node::Node::Text("Goodbye, ".to_string()),
                node::Node::Styled(
                    node::Style::Italic,
                    Box::new(node::Node::Reference(node::Reference {
                        target: crate::core::entity::Id::with_id("b"),
                        caption: vec![node::Node::Text("world".to_string())],
                    })),
                ),
                node::Node::Text("!".to_string()),
            ],
        };

        let elements = vec![
            block::Block::Heading(block::Heading::new(1, vec![node::Node::Text("Hello, world!".to_string())])),
            block::Block::Paragraph(paragraph),
            block::Block::Line,
            block::Block::Paragraph(paragraph2),
            block::Block::Heading(block::Heading::new(2, vec![node::Node::Text("Goodbye, world!".to_string())])),
        ];

        Document { blocks: elements }
    }

    pub fn with_blocks(blocks: Blocks) -> Self {
        Document { blocks }
    }
}
