
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Heading {
    pub level: u8,
    pub nodes: super::Nodes,
}

impl Heading {
    pub fn new(level: u8, text: super::Nodes) -> Heading {
        Heading { level, nodes: text }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CodeBlock {
    pub language: Option<String>,
    pub code: String,
}

pub mod callout {
    use crate::core::entity::zettel::document::Blocks;


    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Callout {
        pub kind: Kind,
        pub blocks: Blocks,
    }

    impl Callout {
        pub fn new(kind: Kind, blocks: Blocks) -> Callout {
            Callout { kind, blocks }
        }
    }
}

pub mod bullet_point {
    use super::super::Nodes;

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct TaskInfo {

    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct BulletPoint {
        pub task_info: Option<TaskInfo>,
        pub nodes: Nodes,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Paragraph {
    pub nodes: super::Nodes,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Block {
    Heading(Heading),
    Line,
    CodeBlock(CodeBlock),
    Callout(callout::Callout),
    BulletPoint(bullet_point::BulletPoint),
    Paragraph(Paragraph),
}
