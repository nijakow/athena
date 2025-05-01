use crate::{core::entity, util::hashing};


pub struct GlobalCache {
    hashes: std::collections::HashMap<std::path::PathBuf, hashing::Sha256>,
    titles: std::collections::HashMap<entity::Id, String>,
}

impl GlobalCache {
    pub fn new() -> Self {
        Self {
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
}
