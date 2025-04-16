
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Sha256 {
    bytes: [u8; 32],
}

impl Sha256 {
    pub(crate) fn new(bytes: [u8; 32]) -> Self {
        Self { bytes }
    }

    pub(crate) fn from_string<S: ToString>(string: S) -> Self {
        let bytes = hex::decode(string.to_string()).unwrap();
        let mut sha256_bytes = [0; 32];
        sha256_bytes.copy_from_slice(&bytes);
        Sha256::new(sha256_bytes)
    }

    pub(crate) fn from_sha256_digest<D: sha2::Digest>(digest: D) -> Sha256 {
        let bytes = digest.finalize();
        let mut sha256_bytes = [0; 32];
        sha256_bytes.copy_from_slice(&bytes);
        Sha256::new(sha256_bytes)
    }

    pub(crate) fn hash_bytes(bytes: &[u8]) -> Self {
        use sha2::Digest;

        let mut hasher = sha2::Sha256::new();

        hasher.update(bytes);

        Sha256::from_sha256_digest(hasher)
    }

    pub(crate) fn as_string(&self) -> String {
        hex::encode(&self.bytes)
    }
}

impl serde::Serialize for Sha256 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.as_string())
    }
}

impl<'de> serde::Deserialize<'de> for Sha256 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Sha256::from_string(s))
    }
}
