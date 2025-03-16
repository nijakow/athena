
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Heading {
    pub level: u8,
    pub text: super::Nodes,
}

impl Heading {
    pub fn new(level: u8, text: super::Nodes) -> Heading {
        Heading { level, text }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct CodeBlock {
    pub language: Option<String>,
    pub code: String,
}

pub mod callout {
    use crate::core::zettel::document::Blocks;


    #[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
    pub enum Kind {
        Basic,
        Quote,
        Note,
        Warning,
        Info,
        Error,
    }

    impl From<&str> for Kind {
        fn from(s: &str) -> Kind {
            match s {
                "quote"   => Kind::Quote,
                "note"    => Kind::Note,
                "warning" => Kind::Warning,
                "info"    => Kind::Info,
                "error"   => Kind::Error,
                _         => Kind::Basic,
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
    pub struct Callout {
        pub kind: Kind,
        pub blocks: Blocks,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Paragraph {
    pub nodes: super::Nodes,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Block {
    Heading(Heading),
    Line,
    CodeBlock(CodeBlock),
    Callout(callout::Callout),
    Paragraph(Paragraph),
}
