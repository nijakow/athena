
use crate::semantic;

use super::Nodes;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Style {
    Bold,
    Italic,
    Underline,
    Strikethrough,
}

pub mod reference {
    use crate::core::entity;
    use super::Nodes;

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum ReferenceTarget {
        Entity(entity::Id),
        Url(url::Url),
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Reference {
        pub target: ReferenceTarget,
        pub caption: Nodes,
        pub embed: bool,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Node {
    Newline,
    Text(String),
    Tag(String),
    Code(String),
    Styled(Style, Box<Node>),
    Reference(reference::Reference),
    Grouped(Nodes),
}

impl semantic::Scannable for Node {
    fn iterate_info_items<F: FnMut(semantic::InfoItem)>(&self, func: &mut F) {
        match self {
            Node::Text(text) => {}
            Node::Tag(tag) => {
                func(semantic::InfoItem::Tag(tag.clone()));
            }
            Node::Styled(_, node) => node.iterate_info_items(func),
            Node::Reference(reference) => {
                func(semantic::InfoItem::Link(reference.target.clone()));
                reference.caption.iterate_info_items(func);
            }
            Node::Grouped(nodes) => nodes.iterate_info_items(func),
            _ => {}
        }
    }
}
