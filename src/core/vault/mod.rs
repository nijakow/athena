use std::collections::HashMap;

use super::{config, zettel};

mod files;

struct ZettelInfo {
    title: String,
}

impl ZettelInfo {
    fn new(title: String) -> ZettelInfo {
        ZettelInfo { title }
    }

    fn title(&self) -> &str {
        &self.title
    }
}

pub struct Vault {
    files: files::Files,
    zettels: HashMap<zettel::Id, ZettelInfo>,
}

pub type VaultOpenResult = Result<Vault, ()>;

impl Vault {
    fn new(config: config::Config) -> Vault {
        let dummy_zettels = vec![
            ("a", "First zettel"),
            ("b", "Second zettel"),
            ("c", "Third zettel"),
        ];

        let zettels = dummy_zettels
            .into_iter()
            .map(|(id, title)| (zettel::Id::with_id(id), ZettelInfo::new(title.to_string())))
            .collect();

        Vault {
            files: files::Files::new(config.vault_path.unwrap()),
            zettels,
        }
    }

    pub(crate) fn open(config: config::Config) -> VaultOpenResult {
        Ok(Self::new(config))
    }

    pub fn list_zettels(&self) -> Vec<(zettel::Id, &str)> {
        self.zettels
            .iter()
            .map(|(id, info)| (id.clone(), info.title()))
            .collect()
    }

    pub fn load(&self, id: &zettel::Id) -> Option<zettel::Zettel> {
        let is_present = self.zettels.contains_key(id);

        if is_present {
            Some(zettel::Zettel::dummy())
        } else {
            None
        }
    }
}
