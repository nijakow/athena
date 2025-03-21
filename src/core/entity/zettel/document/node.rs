use crate::core::entity;

use super::Nodes;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Style {
    Bold,
    Italic,
    Underline,
    Strikethrough,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Reference {
    pub target: entity::Id,
    pub caption: Nodes,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Node {
    Newline,
    Text(String),
    Tag(String),
    Code(String),
    Styled(Style, Box<Node>),
    Reference(Reference),
    Grouped(Nodes),
}
