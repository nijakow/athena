use crate::semantic;

pub mod document;
pub mod parts;

pub use parts::header::Header;
pub use parts::body::Body;


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
