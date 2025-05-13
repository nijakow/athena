use crate::core::{
    entity,
    vault::{caching, resource, volume::VolumeEnum},
};

use super::super::{flags, info, path, Volume, VolumeId, VolumePath};

pub struct DirectoryVolume {
    id: VolumeId,
    base_path: std::path::PathBuf,
    is_home: bool,
    file_name_cache: std::collections::HashMap<String, std::path::PathBuf>,
}

impl Into<VolumeEnum> for DirectoryVolume {
    fn into(self) -> VolumeEnum {
        VolumeEnum::Directory(self)
    }
}

impl DirectoryVolume {
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

    fn construct_volume_path(&self, path: &std::path::Path) -> Option<path::VolumePath> {
        let path_relative_to_base = path
            .strip_prefix(&self.base_path)
            .ok()
            .map(|p| p.to_path_buf());

        match path_relative_to_base {
            Some(path) => Some(path::VolumePath::new(self.id.clone(), path)),
            None => None,
        }
    }

    pub fn reconstruct_full_path(&self, path: &path::VolumePath) -> Option<std::path::PathBuf> {
        if path.volume() != self.id() {
            return None;
        }

        let full_path = self.base_path.join(path.path());

        Some(full_path)
    }

    fn list_files(&self) -> impl Iterator<Item = std::path::PathBuf> {
        fn condition(
            entry: Result<walkdir::DirEntry, walkdir::Error>,
        ) -> Option<std::path::PathBuf> {
            let entry = entry.unwrap();
            if entry.file_type().is_file() && !DirectoryVolume::is_path_excluded(entry.path()) {
                Some(entry.into_path())
            } else {
                None
            }
        }

        walkdir::WalkDir::new(&self.base_path)
            .into_iter()
            .filter_map(condition)
    }

    pub fn map_resource_func<'a, T>(
        &'a self,
        func: impl Fn(&resource::Resource) -> T + 'a,
    ) -> impl Iterator<Item = T> + 'a {
        self.list_resources().map(move |resource| func(&resource))
    }

    fn file_if_exists<S: ToString>(&self, name: S) -> Option<std::path::PathBuf> {
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
            .map(|path| resource::Resource::from_path(self.construct_volume_path(&path).unwrap()))
    }
}

impl Volume for DirectoryVolume {
    fn id(&self) -> &VolumeId {
        &self.id
    }

    fn list_resources<'a>(&'a self) -> Box<dyn Iterator<Item = resource::Resource> + 'a> {
        Box::new(self.list_files().map(move |path| {
            let vp = self.construct_volume_path(&path).unwrap();
            resource::Resource::from_path(vp)
        }))
    }

    fn resource_by_id(
        &self,
        id: &entity::Id,
        resource_interface: &dyn resource::ResourceInterface,
        cache: &mut caching::GlobalCache,
    ) -> Option<resource::Resource> {
        match id {
            entity::Id::Sha256(sha256) => {
                for resource in self.list_resources() {
                    if let Some(hash) = resource.content_hash(resource_interface, cache) {
                        if hash == *sha256 {
                            return Some(resource);
                        }
                    }
                }

                None
            }
            entity::Id::Email(_) => None, // TODO!
            entity::Id::Basic(name) => self.resource_by_short_name(name),
        }
    }

    fn tick(&self) {}

    fn find_directory(&self, purpose: info::DirectoryPurpose) -> Option<std::path::PathBuf> {
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

    fn open_path(&self, path: &VolumePath) -> Result<Box<dyn std::io::Read>, std::io::Error> {
        let translated = self.reconstruct_full_path(path).ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::NotFound, "Path not found in volume")
        })?;

        std::fs::File::open(translated).map(|f| Box::new(f) as Box<dyn std::io::Read>)
    }
}
