use crate::core::entity;

pub struct Flags {
    pub has_zettels: bool,
}

impl Flags {
    pub fn new() -> Self {
        Flags { has_zettels: false }
    }

    pub fn with_zettels(mut self) -> Self {
        self.has_zettels = true;
        self
    }
}

pub struct Storage {
    pub flags: Flags,
    pub base_path: std::path::PathBuf,
}

impl Storage {
    pub fn new(base_path: std::path::PathBuf, flags: Flags) -> Self {
        Storage { flags, base_path }
    }

    pub fn list_files(&self) -> Vec<std::path::PathBuf> {
        let mut files = vec![];

        for entry in std::fs::read_dir(&self.base_path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.is_file() {
                files.push(path);
            }
        }

        files
    }

    pub fn file<S: ToString>(&self, name: S) -> std::path::PathBuf {
        self.base_path.join(name.to_string())
    }

    pub fn file_if_exists<S: ToString>(&self, name: S) -> Option<std::path::PathBuf> {
        let path = self.file(name);

        if path.exists() {
            Some(path)
        } else {
            None
        }
    }

    pub fn list_entities(&self) -> Vec<entity::Id> {
        self.list_files()
            .iter()
            .filter_map(|path| {
                let id = path.file_stem()?.to_str()?;
                Some(entity::Id::with_id(id))
            })
            .collect()
    }
}
