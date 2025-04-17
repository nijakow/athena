use std::sync::RwLock;

use crate::core::{entity, io::resource};

pub mod cache;
pub mod flags;
pub mod info;


pub type RepoId = crate::util::hashing::Sha256;


pub struct Repository {
    id: RepoId,
    base_path: std::path::PathBuf,
    is_home: bool,
    cache: cache::RepositoryCache,
}

impl Repository {
    pub fn new(base_path: std::path::PathBuf, _flags: flags::Flags) -> Self {
        let id = RepoId::hash_string(base_path.to_string_lossy().to_string());

        let file_name_cache = {
            let mut files = std::collections::HashMap::new();
            let mut dirs = vec![base_path.clone()];

            while let Some(dir) = dirs.pop() {
                for entry in std::fs::read_dir(&dir).unwrap() {
                    let entry = entry.unwrap();
                    let path = entry.path();

                    if path.file_name().unwrap().to_string_lossy().starts_with(".") {
                        continue;
                    }

                    if path.is_dir() {
                        dirs.push(path);
                    } else if path.is_file() {
                        let name = path.file_name().unwrap().to_string_lossy().to_string();
                        files.insert(name, path);
                    }
                }
            }

            files
        };

        let cache_file_path = base_path.join("athena-cache.json");

        let resource_cache = if cache_file_path.exists() {
            resource::cache::ResourceCache::load_from_file(&cache_file_path)
                .unwrap_or_else(|_| resource::cache::ResourceCache::new())
        } else {
            resource::cache::ResourceCache::new()
        };

        Self {
            id,
            base_path,
            is_home: false,
            cache: cache::RepositoryCache {
                cache_file_path,
                file_name_cache,
                resource_cache: RwLock::new(resource_cache),
            },
        }
    }

    pub fn id(&self) -> &RepoId {
        &self.id
    }

    pub fn list_files(&self) -> Vec<std::path::PathBuf> {
        self.cache.file_name_cache.values().cloned().collect()
    }

    pub fn file_if_exists<S: ToString>(&self, name: S) -> Option<std::path::PathBuf> {
        let name = name.to_string();
        self.cache.file_name_cache.get(&name).cloned()
    }

    fn list_resources(&self) -> Vec<resource::Resource> {
        self.list_files()
            .iter()
            .map(|path| resource::Resource::from_path(path.clone()))
            .collect()
    }

    pub fn list_entities(&self) -> Vec<entity::Id> {
        self.list_resources()
            .iter()
            .map(|resource| {
                entity::Id::for_resource(resource, &mut *self.cache.resource_cache.write().unwrap())
            })
            .collect()
    }

    fn file_by_short_name(&self, name: &str) -> Option<std::path::PathBuf> {
        // Try different formats: .zson, .md

        let extensions = resource::Type::all_extensions();

        // If the name contains a dot, and the dot is not the leading character, then we assume
        // that the name already contains a file extension.
        let has_file_extension = name.contains('.') && !name.starts_with('.');

        let variants = if has_file_extension {
            vec![name.to_string()]
        } else {
            extensions
                .iter()
                .map(|ext| format!("{}.{}", name, ext))
                .collect()
        };

        for name in variants {
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
                    if let Some(hash) =
                        resource.content_hash(&mut *self.cache.resource_cache.write().unwrap())
                    {
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

    fn save(&self) {
        let resource_cache = self.cache.resource_cache.read().unwrap();
        resource_cache.save_to_file(&self.cache.cache_file_path).unwrap();
    }

    pub fn tick(&self) {
        self.save();
    }

    pub fn find_directory(&self, purpose: info::DirectoryPurpose) -> Option<std::path::PathBuf> {
        match purpose {
            info::DirectoryPurpose::UserDirectory(info::UserDirectory::Home) => {
                if self.is_home {
                    Some(self.base_path.clone())
                } else {
                    None
                }
            }
            _ => None
        }
    }
}


pub struct Repositories {
    repos: Vec<Repository>,
}

impl Repositories {
    pub fn new(repos: Vec<Repository>) -> Self {
        Repositories { repos }
    }

    pub fn list_entities(&self) -> Vec<entity::Id> {
        self.repos
            .iter()
            .flat_map(|storage| storage.list_entities())
            .collect()
    }

    pub fn find_resource_for_id(&self, id: &entity::Id) -> Option<crate::core::io::resource::Resource> {
        self.repos
            .iter()
            .filter_map(|storage| storage.resource_by_id(id))
            .next()
    }

    pub fn tick(&self) {
        for storage in &self.repos {
            storage.tick();
        }
    }
}
