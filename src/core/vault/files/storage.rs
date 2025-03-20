use crate::core::{entity, io::resource};

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

    fn list_resources(&self) -> Vec<resource::Resource> {
        self.list_files()
            .iter()
            .map(|path| {
                resource::Resource::from_path(path.clone())
            })
            .collect()
    }

    pub fn list_entities(&self) -> Vec<entity::Id> {
        self.list_resources()
            .iter()
            .map(|resource| {
                entity::Id::for_resource(resource)
            })
            .collect()
    }

    fn file_by_short_name(&self, name: &str) -> Option<std::path::PathBuf> {
        // Try different formats: .zson, .md

        let extensions = resource::Type::all_extensions();

        // TODO: Iterate over the storages first, then the extensions

        for ext in extensions {
            let name = format!("{}.{}", name, ext);
            if let Some(path) = self.file_if_exists(&name) {
                return Some(path);
            }
        }

        None
    }

    fn resource_by_short_name(&self, name: &str) -> Option<resource::Resource> {
        self.file_by_short_name(name)
            .map(|path| resource::Resource::from_path(path))
    }

    pub fn resource_by_id(&self, id: &entity::Id) -> Option<resource::Resource> {
        match id {
            entity::Id::Sha256(sha256) => {
                for resource in self.list_resources() {
                    println!("Checking resource: {:?}", resource.path());
                    
                    if let Some(hash) = resource.content_hash() {
                        if hash == *sha256 {
                            return Some(resource);
                        }
                    }
                }

                None
            }
            entity::Id::Basic(name) => self.resource_by_short_name(name),
        }
    }
}
