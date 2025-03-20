
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
