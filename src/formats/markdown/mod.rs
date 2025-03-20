
use url;

pub mod parser;

#[derive(Debug)]
pub enum LinkKind {
    Internal,
    External,
}


#[derive(Debug)]
pub enum LinkTarget {
    Zettel(String),
    Url(url::Url),
    FreeForm(String),
}

impl LinkTarget {
    pub fn guess<S: ToString>(target: S) -> LinkTarget {
        let target = target.to_string();
        
        if let Ok(url) = url::Url::parse(&target) {
            LinkTarget::Url(url)
        } else {
            LinkTarget::FreeForm(target)
        }
    }
}


#[derive(Debug)]
pub struct Link {
    pub kind: LinkKind,
    pub target: LinkTarget,
    pub title: Option<Nodes>,
}

impl Link {
    pub fn with_target(kind: LinkKind, target: LinkTarget) -> Link {
        Link {
            kind,
            target,
            title: None,
        }
    }

    pub fn with_title(kind: LinkKind, target: LinkTarget, title: Nodes) -> Link {
        Link {
            kind,
            target,
            title: Some(title),
        }
    }
}

#[derive(Debug)]
pub enum Node {
    Newline,
    Text(String),
    Bold(Nodes),
    Italic(Nodes),
    Link {
        embed: bool,
        link: Link,
    },
    Code(String),
    Tag(String),
}

pub type Nodes = Vec<Node>;

#[derive(Debug)]
pub struct Heading(pub u8, pub Nodes);

#[derive(Debug, Clone)]
pub enum TaskStatus {
    Completed,
    Pending,
}

#[derive(Debug)]
pub struct BulletPoint(pub Option<TaskStatus>, pub Nodes);

#[derive(Debug)]
pub enum Block {
    ThematicBreak,
    Heading(Heading),
    BulletPoint(BulletPoint),
    Code(Option<String>, String),
    Callout(Blocks),
    Nodes(Nodes),
}

pub type Blocks = Vec<Block>;

/*
 * A markdown document.
 */

#[derive(Debug)]
pub struct Document {
    pub blocks: Blocks,
}
