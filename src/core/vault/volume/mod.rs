use crate::core::entity;
use crate::core::vault::resource;

use super::caching;

pub mod flags;
pub mod info;
pub mod path;

pub type VolumeId = crate::util::hashing::Sha256;
pub type VolumePath = path::VolumePath;

pub struct Volume {
    id: VolumeId,
    base_path: std::path::PathBuf,
    is_home: bool,
    file_name_cache: std::collections::HashMap<String, std::path::PathBuf>,
}

impl Volume {
    fn is_path_excluded(path: &std::path::Path) -> bool {
        // Check if any of the subpaths start with "."
        for component in path.components() {
            if let std::path::Component::Normal(name) = component {
                if name.to_string_lossy().starts_with('.') {
                    return true;
                }
            }
        }
        false
    }

    pub fn new(base_path: std::path::PathBuf, _flags: flags::Flags) -> Self {
        let id = VolumeId::hash_string(base_path.to_string_lossy().to_string());

        let file_name_cache = {
            let mut files = std::collections::HashMap::new();
            let mut dirs = vec![base_path.clone()];

            while let Some(dir) = dirs.pop() {
                for entry in std::fs::read_dir(&dir).unwrap() {
                    let entry = entry.unwrap();
                    let path = entry.path();

                    if Self::is_path_excluded(&path) {
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

        Self {
            id,
            base_path,
            is_home: false,
            file_name_cache,
        }
    }

    pub fn id(&self) -> &VolumeId {
        &self.id
    }

    pub fn construct_volume_path(&self, path: &std::path::Path) -> Option<path::VolumePath> {
        let path_relative_to_base = path
            .strip_prefix(&self.base_path)
            .ok()
            .map(|p| p.to_path_buf());

        match path_relative_to_base {
            Some(path) => Some(path::VolumePath::new(self.id.clone(), path)),
            None => None,
        }
    }

    pub fn list_files(&self) -> impl Iterator<Item = std::path::PathBuf> {
        fn condition(
            entry: Result<walkdir::DirEntry, walkdir::Error>,
        ) -> Option<std::path::PathBuf> {
            let entry = entry.unwrap();
            if entry.file_type().is_file() && !Volume::is_path_excluded(entry.path()) {
                Some(entry.into_path())
            } else {
                None
            }
        }

        walkdir::WalkDir::new(&self.base_path)
            .into_iter()
            .filter_map(condition)
    }

    pub fn list_resources(&self) -> impl Iterator<Item = resource::Resource> {
        self.list_files()
            .map(|path| resource::Resource::from_path(path))
    }

    pub fn map_resource_func<'a, T>(
        &'a self,
        func: impl Fn(&resource::Resource) -> T + 'a,
    ) -> impl Iterator<Item = T> + 'a {
        self.list_resources().map(move |resource| func(&resource))
    }

    pub fn file_if_exists<S: ToString>(&self, name: S) -> Option<std::path::PathBuf> {
        let name = name.to_string();
        self.file_name_cache.get(&name).cloned()
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

    pub fn resource_by_id(&self, id: &entity::Id, cache: &mut caching::GlobalCache) -> Option<resource::Resource> {
        match id {
            entity::Id::Sha256(sha256) => {
                for resource in self.list_resources() {
                    if let Some(hash) = resource.content_hash(cache) {
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

    pub fn tick(&self) {
        // Do nothing
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
            _ => None,
        }
    }
}

pub struct Volumes {
    vols: Vec<Volume>,
}

impl Volumes {
    pub fn new(vols: Vec<Volume>) -> Self {
        Self {
            vols
        }
    }

    pub fn volume_by_id(&self, id: &VolumeId) -> Option<&Volume> {
        self.vols.iter().find(|volume| volume.id() == id)
    }

    pub fn volume_by_id_mut(&mut self, id: &VolumeId) -> Option<&mut Volume> {
        self.vols.iter_mut().find(|volume| volume.id() == id)
    }

    pub fn list_resources<'a>(&'a self) -> impl Iterator<Item = resource::Resource> + 'a {
        self.vols
            .iter()
            .flat_map(|storage| storage.list_resources())
    }

    pub fn map_resource_func<'a, T>(
        &'a self,
        func: impl Fn(&resource::Resource) -> T + Clone + 'a,
    ) -> impl Iterator<Item = T> + 'a {
        self.vols
            .iter()
            .flat_map(move |storage| storage.map_resource_func(func.clone()))
    }

    pub fn find_resource_for_id(&self, id: &entity::Id, cache: &mut caching::GlobalCache) -> Option<resource::Resource> {
        self.vols
            .iter()
            .filter_map(|storage| storage.resource_by_id(id, cache))
            .next()
    }

    pub fn tick(&self) {
        for storage in &self.vols {
            storage.tick();
        }
    }
}
