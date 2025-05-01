use crate::semantic;


pub mod conversions;

pub mod block;
pub mod node;


pub type Blocks = Vec<block::Block>;
pub type Nodes = Vec<node::Node>;

impl semantic::Scannable for Blocks {
    fn iterate_info_items<F: FnMut(semantic::InfoItem)>(&self, func: &mut F) {
        for block in self {
            block.iterate_info_items(func);
        }
    }
}

impl semantic::Scannable for Nodes {
    fn iterate_info_items<F: FnMut(semantic::InfoItem)>(&self, func: &mut F) {
        for node in self {
            node.iterate_info_items(func);
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
                    Box::new(node::Node::Reference(node::reference::Reference {
                        target: node::reference::ReferenceTarget::Entity(crate::core::entity::Id::with_id("b")),
                        caption: vec![node::Node::Text("world".to_string())],
                        embed: false,
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

impl semantic::Scannable for Document {
    fn iterate_info_items<F: FnMut(semantic::InfoItem)>(&self, func: &mut F) {
        for block in &self.blocks {
            block.iterate_info_items(func);
        }
    }
}
