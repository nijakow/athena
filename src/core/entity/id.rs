
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Sha256 {
    bytes: [u8; 32],
}

impl Sha256 {
    pub(crate) fn new(bytes: [u8; 32]) -> Sha256 {
        Sha256 { bytes }
    }

    pub(crate) fn as_string(&self) -> String {
        hex::encode(&self.bytes)
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Id {
    Sha256(Sha256),
    Basic(String),
}

impl serde::Serialize for Id {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let stringified = match self {
            Id::Sha256(sha256) => sha256.as_string(),
            Id::Basic(string) => string.clone(),
        };

        stringified.serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for Id {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let id = String::deserialize(deserializer)?;
        Ok(Id::from_string(id))
    }
}


impl Id {
    pub(crate) fn from_sha256(sha: Sha256) -> Id {
        Id::Sha256(sha)
    }

    pub(crate) fn from_basic<S: ToString>(string: S) -> Id {
        Id::Basic(string.to_string())
    }

    pub(crate) fn from_string<S: ToString>(string: S) -> Id {
        let string = string.to_string();

        if string.len() == 64 {
            let bytes = hex::decode(string).unwrap();
            let mut sha256_bytes = [0; 32];
            sha256_bytes.copy_from_slice(&bytes);
            Id::from_sha256(Sha256::new(sha256_bytes))
        } else {
            Id::from_basic(string)
        }
    }

    pub(crate) fn with_id<S: ToString>(id: S) -> Id {
        Id::from_string(id)
    }

    pub fn id(&self) -> String {
        match self {
            Id::Sha256(sha256) => sha256.as_string(),
            Id::Basic(string) => string.clone(),
        }
    }

    pub fn as_safe_uri(&self) -> String {
        format!("/entity/{}", self.id())
    }
}
