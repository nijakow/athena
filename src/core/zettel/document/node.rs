use crate::core::zettel;

use super::Nodes;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Style {
    Bold,
    Italic,
    Underline,
    Strikethrough,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Link {
    pub target: zettel::Id,
    pub caption: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Node {
    Text(String),
    Styled(Style, Box<Node>),
    Link(Link),
    Grouped(Nodes),
}
