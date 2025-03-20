use crate::core::entity;

pub mod storage;

pub struct Files {
    storages: Vec<storage::Storage>,
}

impl Files {
    pub fn new(storages: Vec<storage::Storage>) -> Self {
        Files { storages }
    }

    pub fn list_entities(&self) -> Vec<entity::Id> {
        self.storages
            .iter()
            .flat_map(|storage| storage.list_entities())
            .collect()
    }

    pub fn find_resource_for_id(&self, id: &entity::Id) -> Option<crate::core::io::resource::Resource> {
        self.storages
            .iter()
            .filter_map(|storage| storage.resource_by_id(id))
            .next()
    }
}
