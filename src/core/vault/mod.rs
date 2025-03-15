use std::collections::HashMap;

use super::{config, zettel};

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
    zettels: HashMap<zettel::Id, ZettelInfo>,
}

pub type VaultOpenResult = Result<Vault, ()>;

impl Vault {
    fn new() -> Vault {
        let dummy_zettels = vec![
            ("a", "First zettel"),
            ("b", "Second zettel"),
            ("c", "Third zettel"),
        ];

        let zettels = dummy_zettels
            .into_iter()
            .map(|(id, title)| (zettel::Id::with_id(id), ZettelInfo::new(title.to_string())))
            .collect();

        Vault { zettels }
    }

    pub(crate) fn open(_config: config::Config) -> VaultOpenResult {
        Ok(Self::new())
    }

    pub fn list_zettels(&self) -> Vec<(zettel::Id, &str)> {
        self.zettels
            .iter()
            .map(|(id, info)| (id.clone(), info.title()))
            .collect()
    }
}
