use crate::{core::vault::{resource, volume::{path, Volume, VolumeEnum, VolumeId}}, util::hashing};


pub struct EmailVolume {
    id: VolumeId,
    base_path: std::path::PathBuf,
}

impl Into<VolumeEnum> for EmailVolume {
    fn into(self) -> VolumeEnum {
        VolumeEnum::Email(self)
    }
}

impl EmailVolume {
    pub fn new(base_path: std::path::PathBuf) -> Self {
        let id = VolumeId::hash_string(base_path.to_string_lossy().to_string());
        Self { id, base_path }
    }

    pub fn base_path(&self) -> &std::path::PathBuf {
        &self.base_path
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

    fn find_path_for_email(&self, hash: &hashing::Sha256) -> Option<std::path::PathBuf> {
        let try_extension = |ext: &str| {
            let mut path = self.base_path.clone();
            let stringified = hash.as_string();
            let first_two_chars = &stringified[0..2];
            path.push(first_two_chars);
            path.push(format!("{}.{}", hash.as_string(), ext));
            if path.exists() {
                Some(path)
            } else {
                None
            }
        };

        // Try .eml and .eml.gz
        try_extension("eml").or_else(|| try_extension("eml.gz"))
    }
}

impl Volume for EmailVolume {
    fn id(&self) -> &crate::core::vault::volume::VolumeId {
        &self.id
    }

    fn list_resources<'a>(&'a self) -> Box<dyn Iterator<Item = resource::Resource> + 'a> {
        Box::new(self.base_path.read_dir().unwrap().filter_map(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                let vp = self.construct_volume_path(&path).unwrap();
                Some(resource::Resource::from_path(vp))
            } else {
                None
            }
        }))
    }

    fn resource_by_id(
        &self,
        id: &crate::core::entity::Id,
        _resource_interface: &dyn crate::core::vault::resource::ResourceInterface,
        _cache: &mut crate::core::vault::caching::GlobalCache,
    ) -> Option<crate::core::vault::resource::Resource> {
        match id {
            crate::core::entity::Id::Sha256(_) => None,
            crate::core::entity::Id::Email(sha256) => {
                if let Some(path) = self.find_path_for_email(sha256) {
                    let vp = self.construct_volume_path(&path).unwrap();
                    return Some(resource::Resource::from_path(vp));
                }
                None
            }
            crate::core::entity::Id::Basic(_) => None,
        }
    }

    fn tick(&self) {
        todo!()
    }

    fn find_directory(&self, _purpose: crate::core::vault::volume::info::DirectoryPurpose) -> Option<std::path::PathBuf> {
        None
    }

    fn open_path(&self, path: &crate::core::vault::volume::VolumePath) -> Result<Box<dyn std::io::Read>, std::io::Error> {
        let translated = self.reconstruct_full_path(path).ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::NotFound, "Path not found in volume")
        })?;

        std::fs::File::open(translated).map(|f| Box::new(f) as Box<dyn std::io::Read>)
    }
}

