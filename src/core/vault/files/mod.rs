
pub mod storage;


pub struct Files {
    storages: Vec<storage::Storage>,
}

impl Files {
    pub fn new(storages: Vec<storage::Storage>) -> Self {
        Files { storages }
    }

    pub fn list_files(&self) -> Vec<std::path::PathBuf> {
        self.storages.iter().flat_map(|storage| storage.list_files()).collect()
    }

    pub fn find_file<S: ToString>(&self, name: S) -> Option<std::path::PathBuf> {
        let name = name.to_string();
        
        for storage in &self.storages {
            if let Some(path) = storage.file_if_exists(&name) {
                return Some(path);
            }
        }

        None
    }

    pub fn file_by_id(&self, id: &crate::core::entity::zettel::Id) -> Option<std::path::PathBuf> {
        // Try different formats: .zson, .md
        
        for ext in &["zson", "md"] {
            let name = format!("{}.{}", id.id(), ext);
            if let Some(path) = self.find_file(&name) {
                return Some(path);
            }
        }

        None
    }
}
