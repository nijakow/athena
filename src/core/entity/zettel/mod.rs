use crate::semantic;


pub mod document;

pub struct Header {
    pub title: Option<String>,
    pub yaml: Option<yaml_rust2::Yaml>,
}

impl Header {
    fn new(title: Option<String>) -> Self {
        Self {
            title,
            yaml: None,
        }
    }

    fn from_yaml(yaml: yaml_rust2::Yaml) -> Self {
        Self {
            title: yaml["title"].as_str().map(|s| s.to_string()),
            yaml: Some(yaml.clone()),
        }
    }
}

impl Default for Header {
    fn default() -> Self {
        Self::new(None)
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

    pub(crate) fn from_obsidian_markdown(document: crate::formats::markdown::ObsidianDocument) -> Result<Zettel, ()> {
        let (yaml, document) = (document.head, document.body);
        
        let head = if let Some(yaml) = yaml {
            Header::from_yaml(yaml)
        } else {
            Header::default()
        };
        let body = Body::from_obsidian_markdown(&document)?;

        Ok(Zettel::new(head, body))
    }

    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn body(&self) -> &Body {
        &self.body
    }

    pub fn title(&self) -> Option<&str> {
        self.header.title.as_deref()
    }

    pub fn body_as_document(&self) -> Option<&document::Document> {
        self.body.as_document()
    }
}

impl semantic::Scannable for Zettel {

    fn iterate_info_items<F: FnMut(semantic::InfoItem)>(&self, func: &mut F) {
        match &self.body {
            Body::Document(doc) => doc.iterate_info_items(func),
            _ => {}
        }
    }
}
