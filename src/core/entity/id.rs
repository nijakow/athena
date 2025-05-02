use crate::core::vault::{self, caching, resource};

use crate::util::hashing::Sha256;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Id {
    Sha256(Sha256),
    Basic(String),
}

impl Id {
    pub(crate) fn from_sha256(sha: Sha256) -> Id {
        Id::Sha256(sha)
    }

    pub(crate) fn from_basic<S: ToString>(string: S) -> Id {
        Id::Basic(string.to_string())
    }

    pub(crate) fn from_string<S: ToString>(string: S) -> Result<Id, ()> {
        let string = string.to_string();

        // If the string starts with "sha256-" then it's a SHA256 hash

        if string.starts_with("sha256-") {
            let sha256_string = string.trim_start_matches("sha256-");
            let sha256 = Sha256::from_string(sha256_string)?;

            Ok(Id::Sha256(sha256))
        } else {
            Ok(Id::Basic(string))
        }
    }

    pub(crate) fn with_id<S: ToString>(id: S) -> Result<Id, ()> {
        Id::from_string(id)
    }

    pub(crate) fn for_resource(
        resource: &vault::resource::Resource,
        cache: &mut caching::GlobalCache,
    ) -> Id {
        if let Some(hash) = resource.content_hash(cache) {
            Id::from_sha256(hash.clone())
        } else {
            let file_name_without_extension = resource
                .volume_path()
                .path()
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .to_string();

            Id::from_basic(file_name_without_extension)
        }
    }

    pub fn id(&self) -> String {
        match self {
            Id::Sha256(sha256) => format!("sha256-{}", sha256.as_string()),
            Id::Basic(string) => string.clone(),
        }
    }

    pub fn as_readable_string(&self) -> String {
        match self {
            Id::Sha256(sha256) => sha256.as_string(),
            Id::Basic(string) => string.clone(),
        }
    }

    pub fn as_safe_uri(&self) -> String {
        format!("/entity/{}", self.id())
    }

    pub fn as_safe_download_uri(&self) -> String {
        format!("/raw/{}", self.id())
    }

    pub fn as_hash(&self) -> Sha256 {
        Sha256::hash_string(self.id())
    }
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
        let id = Id::from_string(id).map_err(|_| serde::de::Error::custom("Invalid ID format"))?;
        Ok(id)
    }
}

impl Into<Sha256> for Id {
    fn into(self) -> Sha256 {
        self.as_hash()
    }
}


pub struct TypedId(pub Id, pub Option<resource::Type>);

impl TypedId {
    pub fn new(id: Id, resource_type: Option<resource::Type>) -> Self {
        Self(id, resource_type)
    }

    pub fn id(&self) -> &Id {
        &self.0
    }

    pub fn resource_type(&self) -> Option<&resource::Type> {
        self.1.as_ref()
    }

    pub fn parse(string: &str) -> Result<(Id, Option<resource::Type>), ()> {
        // Split into main part and extension (taking the last dot as the separator)

        let (main_part, extension) = if let Some(pos) = string.rfind('.') {
            (&string[..pos], Some(&string[pos + 1..]))
        } else {
            (string, None)
        };

        let parsed = Id::from_string(main_part);

        match parsed {
            Ok(id) => Ok((id, extension.and_then(resource::Type::from_extension))),
            Err(_) => Err(()),
        }
    }
}
