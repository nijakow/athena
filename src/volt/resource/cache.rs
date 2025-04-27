
pub struct ResourceCache {
    hashes: std::collections::HashMap<std::path::PathBuf, crate::util::hashing::Sha256>,
}

impl ResourceCache {
    pub fn new() -> Self {
        Self {
            hashes: std::collections::HashMap::new(),
        }
    }

    pub fn load_from_file(path: &std::path::Path) -> std::io::Result<Self> {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let hashes: std::collections::HashMap<std::path::PathBuf, crate::util::hashing::Sha256> =
            serde_json::from_reader(reader)?;

        Ok(Self { hashes })
    }

    pub fn save_to_file(&self, path: &std::path::Path) -> std::io::Result<()> {
        let json = serde_json::to_string(&self.hashes)?;

        {
            use std::io::Write;

            let mut file = std::fs::File::create(path)?;
            file.write_all(json.as_bytes())?;
        }

        Ok(())
    }

    pub fn get_hash(&self, path: &std::path::Path) -> Option<&crate::util::hashing::Sha256> {
        // TODO: Check if the file has been modified since the hash was calculated
        self.hashes.get(path)
    }

    pub fn set_hash(&mut self, path: std::path::PathBuf, hash: crate::util::hashing::Sha256) {
        self.hashes.insert(path, hash);
    }
}


pub struct ResourceCacheSnapshot {
    pub hashes: std::collections::HashMap<std::path::PathBuf, crate::util::hashing::Sha256>,
}

impl crate::util::snapshotting::Snapshottable for ResourceCache {
    type Snapshot = ResourceCacheSnapshot;
    
    fn from_snapshot(snapshot: Self::Snapshot) -> Self {
        ResourceCache {
            hashes: snapshot.hashes,
        }
    }
    
    fn take_snapshot(&self) -> Self::Snapshot {
        ResourceCacheSnapshot {
            hashes: self.hashes.clone(),
        }
    }
}

impl serde::Serialize for ResourceCacheSnapshot {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.hashes.serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for ResourceCacheSnapshot {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let hashes = std::collections::HashMap::<std::path::PathBuf, crate::util::hashing::Sha256>::deserialize(deserializer)?;
        Ok(ResourceCacheSnapshot { hashes })
    }
}
