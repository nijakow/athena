use super::vault;


pub(crate) struct Config {
    pub vault_path: Option<std::path::PathBuf>,
}

impl Config {
    pub fn new() -> Self {
        Self { vault_path: None }
    }
}


pub struct ConfigBuilder {
    config: Config,
}

impl ConfigBuilder {
    pub(crate) fn new() -> Self {
        Self {
            config: Config::new(),
        }
    }

    pub fn vault_path(mut self, path: std::path::PathBuf) -> Self {
        self.config.vault_path = Some(path);
        self
    }

    fn build(self) -> Config {
        self.config
    }

    pub fn open_vault(self) -> vault::VaultOpenResult {
        vault::Vault::open(self.build())
    }
}
