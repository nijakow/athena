
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
    Callout(Option<String>, Blocks),
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

pub struct ObsidianDocument {
    pub head: Option<yaml_rust2::Yaml>,
    pub body: Document,
}

impl From<ObsidianDocument> for Document {
    fn from(obsidian: ObsidianDocument) -> Document {
        obsidian.body
    }
}

impl From<Document> for ObsidianDocument {
    fn from(doc: Document) -> ObsidianDocument {
        ObsidianDocument {
            head: None,
            body: doc,
        }
    }
}


pub fn parse_obsidian_markdown(
    content: crate::core::io::resource::file::FileContent,
) -> Result<ObsidianDocument, ()> {
    match content.extract_content_as_string() {
        Some(content) => {

            let (metadata, content) = crate::util::split_metadata_from_content(content);

            let metadata = metadata
                .and_then(|m| yaml_rust2::YamlLoader::load_from_str(&m).ok())
                .and_then(|mut docs| docs.pop());

            match crate::formats::markdown::parser::parse_document(content) {
                Ok(document) => {
                    Ok(crate::formats::markdown::ObsidianDocument {
                        head: metadata,
                        body: document,
                    })
                }
                Err(e) => Err(()),
            }
        }
        None => Err(()),
    }
}
