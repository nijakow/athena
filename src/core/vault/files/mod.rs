
pub struct Files {
    base: std::path::PathBuf,
}

impl Files {
    pub fn new(base: std::path::PathBuf) -> Self {
        Files { base }
    }

    pub fn list_files(&self) -> Vec<std::path::PathBuf> {
        let mut files = vec![];

        for entry in std::fs::read_dir(&self.base).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.is_file() {
                files.push(path);
            }
        }

        files
    }

    pub fn file<S: ToString>(&self, name: S) -> std::path::PathBuf {
        self.base.join(name.to_string())
    }

    fn file_if_exists<S: ToString>(&self, name: S) -> Option<std::path::PathBuf> {
        let path = self.file(name);

        if path.exists() {
            Some(path)
        } else {
            None
        }
    }

    pub fn file_by_id(&self, id: &crate::core::entity::zettel::Id) -> std::path::PathBuf {
        // Try different formats: .zson, .md
        
        if let Some(path) = self.file_if_exists(format!("{}.md", id.id())) {
            return path;
        }

        self.file(format!("{}.zson", id.id()))
    }
}
