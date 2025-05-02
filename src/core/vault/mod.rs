use super::{
    config,
    entity::{self, zettel},
};

use crate::{core::vault, semantic};

pub mod caching;
pub mod event;
pub mod resource;
pub mod volume;

pub struct Vault {
    volumes: vault::volume::Volumes,
    cache: std::sync::RwLock<caching::GlobalCache>,
}

pub type VaultOpenResult = Result<Vault, ()>;

impl Vault {
    pub(crate) fn open(config: config::Config) -> VaultOpenResult {
        let cache_path = config.cache_path();

        // Try to create the cache directory if it doesn't exist
        if !cache_path.exists() {
            std::fs::create_dir_all(&cache_path)
                .map_err(|_| {
                    eprintln!("Unable to create cache directory at {:?}", cache_path);
                    ()
                })?;
        }

        let volumes = vec![vault::volume::Volume::new(
            config.vault_path.unwrap(),
            vault::volume::flags::Flags::new().with_zettels(),
        )];

        let vault = Self {
            volumes: vault::volume::Volumes::new(volumes),
            cache: std::sync::RwLock::new(caching::GlobalCache::new(cache_path)),
        };

        Ok(vault)
    }

    pub fn list_entities(&self) -> Vec<entity::Id> {
        match self.cache.write() {
            Ok(mut cache) => self
                .volumes
                .list_resources()
                .map(move |resource| entity::Id::for_resource(&resource, &mut cache))
                .collect::<Vec<_>>(),
            Err(_) => vec![],
        }
    }

    fn find_resource_for_id(&self, id: &entity::Id) -> Option<vault::resource::Resource> {
        match self.cache.write() {
            Ok(mut cache) => self.volumes.find_resource_for_id(id, &mut cache),
            Err(_) => None,
        }
    }

    pub fn load_resource(&self, id: &entity::Id) -> Option<vault::resource::Resource> {
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
        let perhaps_title = match self.cache.write() {
            Ok(mut cache) => cache.get_title(id),
            Err(_) => None,
        };

        if let Some(title) = perhaps_title {
            return Some(title);
        }

        let entity = self.load_entity(id)?;

        let title = match entity {
            entity::Entity::File(file) => file.metadata().title(),
            entity::Entity::Zettel(zettel) => zettel
                .header()
                .title
                .clone()
                .or_else(|| Some(id.id().to_string())),
        };

        if let Some(title) = &title {
            if let Ok(mut cache) = self.cache.write() {
                cache.set_title(id.clone(), title.clone());
            }
        }

        title
    }

    pub fn tick(&self) {
        self.volumes.tick();
        
        if let Ok(mut cache) = self.cache.write() {
            cache.save().ok();
        }
    }
}

impl semantic::Scannable for Vault {
    fn iterate_info_items<F: FnMut(semantic::InfoItem)>(&self, func: &mut F) {
        for entity in self.list_entities() {
            if let Some(entity) = self.load_entity(&entity) {
                entity.iterate_info_items(func);
            }
        }
    }
}
