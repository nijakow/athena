use crate::core::vault::{resource, volume::{Volume, VolumeEnum}};


pub struct EmailVolume {
    base_path: std::path::PathBuf,
}

impl Into<VolumeEnum> for EmailVolume {
    fn into(self) -> VolumeEnum {
        VolumeEnum::Email(self)
    }
}

impl Volume for EmailVolume {
    fn id(&self) -> &crate::core::vault::volume::VolumeId {
        todo!()
    }

    fn list_resources<'a>(&'a self) -> Box<dyn Iterator<Item = resource::Resource> + 'a> {
        todo!()
    }

    fn resource_by_id(
        &self,
        id: &crate::core::entity::Id,
        resource_interface: &dyn crate::core::vault::resource::ResourceInterface,
        cache: &mut crate::core::vault::caching::GlobalCache,
    ) -> Option<crate::core::vault::resource::Resource> {
        todo!()
    }

    fn tick(&self) {
        todo!()
    }

    fn find_directory(&self, purpose: crate::core::vault::volume::info::DirectoryPurpose) -> Option<std::path::PathBuf> {
        todo!()
    }

    fn open_path(&self, path: &crate::core::vault::volume::VolumePath) -> Result<Box<dyn std::io::Read>, std::io::Error> {
        todo!()
    }
}

