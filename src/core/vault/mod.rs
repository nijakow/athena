use super::{
    config,
    entity::{self, zettel},
    io::resource,
};

mod files;

pub struct Vault {
    files: files::Files,
}

pub type VaultOpenResult = Result<Vault, ()>;

impl Vault {
    fn new(config: config::Config) -> Vault {
        Vault {
            files: files::Files::new(vec![files::storage::Storage::new(
                config.vault_path.unwrap(),
                files::storage::Flags::new().with_zettels(),
            )]),
        }
    }

    pub(crate) fn open(config: config::Config) -> VaultOpenResult {
        Ok(Self::new(config))
    }

    pub fn list_entities(&self) -> Vec<entity::Id> {
        self.files.list_entities()
    }

    pub fn list_zettels(&self) -> Vec<entity::Id> {
        self.list_entities()
    }

    fn find_resource_for_id(&self, id: &entity::Id) -> Option<resource::Resource> {
        let file = self.files.file_by_id(&id)?;

        if file.exists() {
            Some(resource::Resource::from_path(file))
        } else {
            None
        }
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
        self.load_zettel_header(id)
            .map(|header| header.title)
            .flatten()
    }
}
