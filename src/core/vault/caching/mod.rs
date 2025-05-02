use crate::{core::entity, util::hashing};


pub mod storage;


#[derive(serde::Serialize, serde::Deserialize)]
pub struct Metadata {
    paths: std::collections::HashSet<std::path::PathBuf>,
}

impl Metadata {
    pub fn new() -> Self {
        Self {
            paths: std::collections::HashSet::new(),
        }
    }

    pub fn add_path(&mut self, path: std::path::PathBuf) {
        self.paths.insert(path);
    }

    pub fn paths(&self) -> impl std::iter::Iterator<Item = &std::path::PathBuf> {
        self.paths.iter()
    }
}

impl Default for Metadata {
    fn default() -> Self {
        Self::new()
    }
}



#[derive(serde::Serialize, serde::Deserialize)]
struct GlobalCacheSaveState {
    titles: std::collections::HashMap<entity::Id, String>,
    hashes: std::collections::HashMap<std::path::PathBuf, hashing::Sha256>,
}


pub struct GlobalCache {
    base_path: std::path::PathBuf,
    titles: std::collections::HashMap<entity::Id, String>,
    hashes: std::collections::HashMap<std::path::PathBuf, hashing::Sha256>,
    metadata: storage::DataStorage<Metadata>,
}

impl GlobalCache {
    pub fn new(base_path: std::path::PathBuf) -> Self {
        let metadata_path = base_path.join("metadata");
        
        Self {
            base_path,
            titles: std::collections::HashMap::new(),
            hashes: std::collections::HashMap::new(),
            metadata: storage::DataStorage::open(
                metadata_path,
                true,
            ).unwrap()
        }
    }

    pub fn get_hash(&self, path: &std::path::Path) -> Option<&hashing::Sha256> {
        self.hashes.get(path)
    }

    fn report_hash(&mut self, hash: &hashing::Sha256, path: &std::path::Path) {
        self.metadata.modify(hash.clone(), |metadata| {
            metadata.add_path(path.to_path_buf());
        }).map_err(|_| {
            eprintln!("Failed to report hash for path: {:?}", path);
        }).ok();
    }

    pub fn set_hash(&mut self, path: std::path::PathBuf, hash: hashing::Sha256) {
        self.report_hash(&hash, &path); // Tell the system where the hash was found
        self.hashes.insert(path, hash);
    }

    pub fn get_title(&self, id: &entity::Id) -> Option<&String> {
        self.titles.get(id)
    }

    pub fn set_title(&mut self, id: entity::Id, title: String) {
        self.titles.insert(id, title);
    }

    pub fn save(&mut self) -> Result<(), std::io::Error> {
        let snapshot = GlobalCacheSaveState {
            hashes: self.hashes.clone(),
            titles: self.titles.clone(),
        };

        {
            let path = self.base_path.join("cache.json");
            let file = std::fs::File::create(&path)?;
            let writer = std::io::BufWriter::new(file);
            serde_json::to_writer(writer, &snapshot)?;
        }

        self.metadata.flush_cache().map_err(|_| {
            eprintln!("Failed to flush metadata cache");
        }).ok();

        Ok(())
    }
}
