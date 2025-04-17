use crate::core::entity;

use crate::core::repo;


pub struct Files {
    repos: Vec<repo::Repository>,
}

impl Files {
    pub fn new(storages: Vec<repo::Repository>) -> Self {
        Files { repos: storages }
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
