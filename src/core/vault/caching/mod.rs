use crate::{core::entity, util::hashing};


pub mod storage;


#[derive(serde::Serialize, serde::Deserialize)]
struct GlobalCacheSaveState {
    hashes: std::collections::HashMap<std::path::PathBuf, hashing::Sha256>,
    titles: std::collections::HashMap<entity::Id, String>,
}


pub struct GlobalCache {
    base_path: std::path::PathBuf,
    hashes: std::collections::HashMap<std::path::PathBuf, hashing::Sha256>,
    titles: std::collections::HashMap<entity::Id, String>,
}

impl GlobalCache {
    pub fn new(base_path: std::path::PathBuf) -> Self {
        Self {
            base_path,
            hashes: std::collections::HashMap::new(),
            titles: std::collections::HashMap::new(),
        }
    }

    pub fn get_hash(&self, path: &std::path::Path) -> Option<&hashing::Sha256> {
        self.hashes.get(path)
    }

    pub fn set_hash(&mut self, path: std::path::PathBuf, hash: hashing::Sha256) {
        self.hashes.insert(path, hash);
    }

    pub fn get_title(&self, id: &entity::Id) -> Option<&String> {
        self.titles.get(id)
    }

    pub fn set_title(&mut self, id: entity::Id, title: String) {
        self.titles.insert(id, title);
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
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
        
        Ok(())
    }
}
