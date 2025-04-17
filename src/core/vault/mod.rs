use super::{
    config,
    entity::{self, zettel},
};

use crate::volt;

pub struct Vault {
    volumes: volt::volume::Volumes,
}

pub type VaultOpenResult = Result<Vault, ()>;

impl Vault {
    fn new(config: config::Config) -> Vault {
        let volumes = vec![
            volt::volume::Volume::new(
                config.vault_path.unwrap(),
                volt::volume::flags::Flags::new().with_zettels(),
            )
        ];

        Vault {
            volumes: volt::volume::Volumes::new(volumes),
        }
    }

    pub(crate) fn open(config: config::Config) -> VaultOpenResult {
        Ok(Self::new(config))
    }

    pub fn list_entities(&self) -> Vec<entity::Id> {
        self.volumes.list_entities()
    }

    fn find_resource_for_id(&self, id: &entity::Id) -> Option<volt::resource::Resource> {
        self.volumes.find_resource_for_id(id)
    }

    pub fn load_resource(&self, id: &entity::Id) -> Option<volt::resource::Resource> {
        self.find_resource_for_id(id)
    }

    pub fn load_entity(&self, id: &entity::Id) -> Option<entity::Entity> {
        let resource = self.find_resource_for_id(id)?;

        entity::Entity::from_resource(resource).ok()
    }

    pub fn load_zettel(&self, id: &entity::Id) -> Option<zettel::Zettel> {
        if let Some(entity::Entity::Zettel(zettel)) = self.load_entity(id) {
            Some(zettel)
        } else {
            None
        }
    }

    pub fn load_zettel_header(&self, id: &entity::Id) -> Option<zettel::Header> {
        self.load_zettel(id).map(|zettel| zettel.header)
    }

    pub fn title_of_entity(&self, id: &entity::Id) -> Option<String> {

        let entity = self.load_entity(id)?;

        match entity {
            entity::Entity::File(file) => file.metadata().title(),
            entity::Entity::Zettel(zettel) => zettel.header().title.clone().or_else(|| Some(id.id().to_string())),
        }
    }

    pub fn tick(&self) {
        self.volumes.tick();
    }
}
