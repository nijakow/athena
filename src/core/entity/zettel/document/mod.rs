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
