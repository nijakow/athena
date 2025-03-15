
use super::{config, io::resource, zettel};

mod files;


pub struct Vault {
    files: files::Files,
}

pub type VaultOpenResult = Result<Vault, ()>;

impl Vault {
    fn new(config: config::Config) -> Vault {
        Vault {
            files: files::Files::new(config.vault_path.unwrap()),
        }
    }

    pub(crate) fn open(config: config::Config) -> VaultOpenResult {
        Ok(Self::new(config))
    }

    pub fn list_zettels(&self) -> Vec<zettel::Id> {
        self.files.list_files().iter().filter_map(|path| {
            let id = path.file_stem()?.to_str()?;
            Some(zettel::Id::with_id(id))
        }).collect()
    }

    fn find_resource_for_id(&self, id: &zettel::Id) -> Option<resource::Resource> {
        let file = self.files.file_by_id(&id);
        
        if file.exists() {
            Some(resource::Resource::from_path(file))
        } else {
            None
        }
    }

    pub fn load(&self, id: &zettel::Id) -> Option<zettel::Zettel> {
        let resource = self.find_resource_for_id(id);

        match resource {
            Some(resource) => zettel::Zettel::from_resource(resource).map_err(|e| {
                eprintln!("Failed to load Zettel: {:?}", e);
            }).ok(),
            None => None,
        }
    }
}
