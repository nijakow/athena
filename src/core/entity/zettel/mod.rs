
pub mod document;


pub struct Header {
    pub title: Option<String>,
}

impl Header {
    fn new(title: Option<String>) -> Self {
        Header { title }
    }
}

pub enum Body {
    Empty,
    Document(Box<document::Document>),
}

impl Body {

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
