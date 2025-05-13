use crate::core::entity;
use crate::core::vault::resource;

use super::caching;

pub mod flags;
pub mod info;
pub mod path;
pub mod volumes;

pub type VolumeId = crate::util::hashing::Sha256;
pub type VolumePath = path::VolumePath;

pub trait Volume {
    fn id(&self) -> &VolumeId;
    fn list_resources<'a>(&'a self) -> impl Iterator<Item = resource::Resource> + 'a;

    fn map_resource_func<'a, T>(
        &'a self,
        func: impl Fn(&resource::Resource) -> T + 'a,
    ) -> impl Iterator<Item = T> + 'a {
        self.list_resources().map(move |resource| func(&resource))
    }

    fn resource_by_id(
        &self,
        id: &entity::Id,
        resource_interface: &dyn resource::ResourceInterface,
        cache: &mut caching::GlobalCache,
    ) -> Option<resource::Resource>;

    fn tick(&self);

    fn find_directory(&self, purpose: info::DirectoryPurpose) -> Option<std::path::PathBuf>;

    fn open_path(&self, path: &VolumePath) -> Result<Box<dyn std::io::Read>, std::io::Error>;
}


pub enum VolumeEnum {
    Directory(volumes::directory::DirectoryVolume),
}

impl Volume for VolumeEnum {
    fn id(&self) -> &VolumeId {
        match self {
            VolumeEnum::Directory(v) => v.id(),
        }
    }

    fn list_resources<'a>(&'a self) -> impl Iterator<Item = resource::Resource> + 'a {
        match self {
            VolumeEnum::Directory(v) => v.list_resources(),
        }
    }

    fn resource_by_id(
        &self,
        id: &entity::Id,
        resource_interface: &dyn resource::ResourceInterface,
        cache: &mut caching::GlobalCache,
    ) -> Option<resource::Resource> {
        match self {
            VolumeEnum::Directory(v) => v.resource_by_id(id, resource_interface, cache),
        }
    }

    fn tick(&self) {
        match self {
            VolumeEnum::Directory(v) => v.tick(),
        }
    }

    fn find_directory(&self, purpose: info::DirectoryPurpose) -> Option<std::path::PathBuf> {
        match self {
            VolumeEnum::Directory(v) => v.find_directory(purpose),
        }
    }

    fn open_path(&self, path: &VolumePath) -> Result<Box<dyn std::io::Read>, std::io::Error> {
        match self {
            VolumeEnum::Directory(v) => v.open_path(path),
        }
    }
}

pub type VolumeArc = std::sync::Arc<VolumeEnum>;

pub struct Volumes {
    vols: Vec<VolumeArc>,
}

impl Volumes {
    pub fn new(vols: Vec<VolumeEnum>) -> Self {
        Self {
            vols: vols.into_iter().map(|v| std::sync::Arc::new(v)).collect(),
        }
    }

    pub fn volume_by_id(&self, id: &VolumeId) -> Option<&VolumeEnum> {
        self.vols.iter().find_map(|volume| {
            if volume.id() == id {
                Some(volume.as_ref())
            } else {
                None
            }
        })
    }

    pub fn volume_by_id_mut(&mut self, id: &VolumeId) -> Option<&mut VolumeEnum> {
        self.vols
            .iter_mut()
            .find_map(|volume| std::sync::Arc::get_mut(volume).filter(|v| v.id() == id))
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

    pub fn find_resource_for_id(
        &self,
        id: &entity::Id,
        resource_interface: &dyn resource::ResourceInterface,
        cache: &mut caching::GlobalCache,
    ) -> Option<resource::Resource> {
        self.vols
            .iter()
            .filter_map(|storage| storage.resource_by_id(id, resource_interface, cache))
            .next()
    }

    pub fn tick(&self) {
        for storage in &self.vols {
            storage.tick();
        }
    }
}
