use crate::volt;


pub struct ResourceCache {
    hashes: std::collections::HashMap<std::path::PathBuf, crate::util::hashing::Sha256>,
}

impl ResourceCache {
    pub fn new() -> Self {
        Self {
            hashes: std::collections::HashMap::new(),
        }
    }

    pub fn get_hash(&self, path: &std::path::Path) -> Option<&crate::util::hashing::Sha256> {
        // TODO: Check if the file has been modified since the hash was calculated
        self.hashes.get(path)
    }

    pub fn set_hash(&mut self, path: std::path::PathBuf, hash: crate::util::hashing::Sha256) {
        self.hashes.insert(path, hash);
    }
}


#[derive(Debug)]
pub struct ResourceCacheSnapshot {
    pub hashes: std::collections::HashMap<volt::volume::path::VolumePath, crate::util::hashing::Sha256>,
}

impl<'a> crate::util::snapshotting::Snapshottable<'a> for ResourceCache {
    type Snapshot = ResourceCacheSnapshot;
    type Parameter = &'a volt::volume::Volume;
    
    fn from_snapshot(&mut self, snapshot: Self::Snapshot) {
        self.hashes.clear();
        for (path, hash) in snapshot.hashes {
            self.hashes.insert(path.path().to_path_buf(), hash);
        }
    }
    
    fn take_snapshot(&self, parameter: Self::Parameter) -> Self::Snapshot {
        let hashes = self
            .hashes
            .iter()
            .filter_map(|(path, hash)| {
                let volume_path = parameter.construct_volume_path(path);
                
                if let Some(volume_path) = volume_path {
                    Some((volume_path, hash.clone()))
                } else {
                    None
                }
            })
            .collect();

        ResourceCacheSnapshot {
            hashes
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
        let hashes = std::collections::HashMap::<volt::volume::path::VolumePath, crate::util::hashing::Sha256>::deserialize(deserializer)?;
        Ok(ResourceCacheSnapshot { hashes })
    }
}
