
use crate::{core::entity::zettel::document, semantic};


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

impl semantic::Scannable for Body {
    fn iterate_info_items<F: FnMut(semantic::InfoItem)>(&self, func: &mut F) {
        match self {
            Body::Document(doc) => doc.iterate_info_items(func),
            _ => {}
        }
    }
}
