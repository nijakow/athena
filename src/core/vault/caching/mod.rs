use crate::util::hashing;


pub struct GlobalCache {
    hashes: std::collections::HashMap<std::path::PathBuf, hashing::Sha256>,
}

impl GlobalCache {
    pub fn new() -> Self {
        Self {
            hashes: std::collections::HashMap::new(),
        }
    }

    pub fn get_hash(&self, path: &std::path::Path) -> Option<&hashing::Sha256> {
        self.hashes.get(path)
    }

    pub fn set_hash(&mut self, path: std::path::PathBuf, hash: hashing::Sha256) {
        self.hashes.insert(path, hash);
    }
}
