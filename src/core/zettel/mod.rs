use super::io::resource;

pub mod document;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Id {
    id: String,
}

impl Id {
    pub fn with_id<S: ToString>(id: S) -> Id {
        Id { id: id.to_string() }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}


pub struct Header {
    
}

impl Header {
    pub(crate) fn from_json(json: &serde_json::Value) -> Result<Header, ()> {
        Ok(Header {})
    }
}

pub struct Body {
    
}

impl Body {
    pub(crate) fn from_json(json: &serde_json::Value) -> Result<Body, ()> {
        Ok(Body {})
    }
}


pub struct Zettel {
    header: Header,
    body: Body,
}

impl Zettel {
    fn new(header: Header, body: Body) -> Zettel {
        Zettel { header, body }
    }

    fn from_serde_json(json: &serde_json::Value) -> Result<Zettel, ()> {
        let body = Body::from_json(json.get("body").ok_or(())?)?;
        let header = Header::from_json(json)?;

        Ok(Zettel::new(header, body))
    }

    pub fn from_resource(resource: resource::Resource) -> Result<Self, Box<dyn std::error::Error>> {
        let metadata = resource.metadata();

        match metadata.resource_type {
            Some(resource::Type::Athena) => {
                let json = resource.read_to_json()?;
                let zettel = Zettel::from_serde_json(&json).map_err(|_| "Failed to parse Zettel")?;

                Ok(zettel)
            }
            None => Err("Unknown resource type".into()),
        }
    }
}
