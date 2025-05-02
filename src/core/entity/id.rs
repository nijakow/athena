
use crate::core::vault::{self, caching};

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

    pub(crate) fn from_string<S: ToString>(string: S) -> Id {
        let string = string.to_string();

        // If the string starts with "sha256-" then it's a SHA256 hash

        if string.starts_with("sha256-") {
            let sha256_string = string.trim_start_matches("sha256-");
            let sha256 = Sha256::from_string(sha256_string);

            Id::Sha256(sha256)
        } else {
            Id::Basic(string)
        }
    }

    pub(crate) fn with_id<S: ToString>(id: S) -> Id {
        Id::from_string(id)
    }

    pub(crate) fn for_resource(resource: &vault::resource::Resource, cache: &mut caching::GlobalCache) -> Id {
        if let Some(hash) = resource.content_hash(cache) {
            Id::from_sha256(hash.clone())
        } else {
            let file_name_without_extension = resource
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
        Sha256::from_string(self.id())
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
        Ok(Id::from_string(id))
    }
}

impl Into<Sha256> for Id {
    fn into(self) -> Sha256 {
        self.as_hash()
    }
}
