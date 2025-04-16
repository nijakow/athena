
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
