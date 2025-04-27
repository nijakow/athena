use super::vault;
use dirs;


pub(crate) struct Config {
    pub snapshot_path: Option<std::path::PathBuf>,
    pub vault_path: Option<std::path::PathBuf>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            snapshot_path: None,
            vault_path: None
        }
    }

    pub fn snapshot_path(&self) -> std::path::PathBuf {
        // Default is ~/.athena-cache
        self.snapshot_path
            .clone()
            .unwrap_or_else(|| {
                let mut path = dirs::home_dir().expect("Unable to determine home directory");
                path.push(".athena-cache");
                path
            })
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

    pub fn snapshot_path(mut self, path: std::path::PathBuf) -> Self {
        self.config.snapshot_path = Some(path);
        self
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
