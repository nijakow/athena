use super::io::resource::{self, SplitJson};

pub mod document;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Id {
    id: String,
}

impl serde::Serialize for Id {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.id.serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for Id {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let id = String::deserialize(deserializer)?;
        Ok(Id { id })
    }
}


impl Id {
    pub(crate) fn with_id<S: ToString>(id: S) -> Id {
        Id { id: id.to_string() }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn as_safe_uri(&self) -> String {
        format!("/zettel/{}", self.id)
    }
}


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

    fn from_split_json(split_json: &SplitJson) -> Result<Zettel, ()> {
        let body = Body::from_json(&split_json.body)?;
        let header = Header::from_json(&split_json.header)?;

        Ok(Zettel::new(header, body))
    }

    pub fn from_resource(resource: resource::Resource) -> Result<Self, Box<dyn std::error::Error>> {
        let metadata = resource.metadata();

        match metadata.resource_type {
            Some(resource::Type::Athena) => {
                let split_json = resource.read_to_split_json()?;
                let zettel = Zettel::from_split_json(&split_json).map_err(|_| "Failed to parse Zettel")?;

                Ok(zettel)
            }
            None => Err("Unknown resource type".into()),
        }
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
