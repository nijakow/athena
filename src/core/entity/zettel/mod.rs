use crate::core::io::resource::{self, SplitJson};

pub mod document;

pub type Id = crate::core::entity::id::Id;


pub struct Header {
    pub title: Option<String>,
}

impl Header {
    fn new(title: Option<String>) -> Self {
        Header { title }
    }

    pub(crate) fn from_json(json: &serde_json::Value) -> Result<Header, ()> {
        let title = json.get("title").and_then(|v| v.as_str()).map(|s| s.to_string());

        Ok(Header::new(title))
    }
}

pub enum Body {
    Empty,
    Document(Box<document::Document>),
}

impl Body {
    pub(crate) fn from_json(json: &serde_json::Value) -> Result<Body, ()> {
        Ok(Body::Document(Box::new(document::conversions::json::json_to_document(&json)?)))
    }

    pub(crate) fn from_obsidian_markdown(markdown: &crate::formats::markdown::Document) -> Result<Body, ()> {
        Ok(Body::Document(Box::new(document::conversions::markdown::markdown_to_document(&markdown)?)))
    }

    pub fn as_document(&self) -> Option<&document::Document> {
        match self {
            Body::Document(doc) => Some(doc),
            _ => None,
        }
    }
}


pub struct Zettel {
    pub header: Header,
    pub body: Body,
}

impl Zettel {
    fn new(header: Header, body: Body) -> Self {
        Zettel { header, body }
    }

    pub(crate) fn from_split_json(split_json: &SplitJson) -> Result<Zettel, ()> {
        let body = Body::from_json(&split_json.body)?;
        let header = Header::from_json(&split_json.header)?;

        Ok(Zettel::new(header, body))
    }

    pub(crate) fn from_obsidian_markdown(markdown: &crate::formats::markdown::Document) -> Result<Zettel, ()> {
        let body = Body::from_obsidian_markdown(markdown)?;
        let header = Header::new(None); // TODO!

        Ok(Zettel::new(header, body))
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn body(&self) -> &Body {
        &self.body
    }

    pub fn body_as_document(&self) -> Option<&document::Document> {
        self.body.as_document()
    }
}
