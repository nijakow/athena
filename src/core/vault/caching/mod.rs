use crate::{core::entity, util::hashing};

use super::volume;

pub mod storage;

pub mod caches {
    pub mod by_path {
        use crate::{core::vault::caching::storage::Stored, util::hashing};

        #[derive(serde::Serialize, serde::Deserialize)]
        pub struct Metadata {
            hash: Option<hashing::Sha256>,
        }

        impl Metadata {
            pub fn new() -> Self {
                Self { hash: None }
            }

            pub fn set_hash(&mut self, hash: hashing::Sha256) {
                self.hash = Some(hash);
            }

            pub fn get_hash(&self) -> Option<&hashing::Sha256> {
                self.hash.as_ref()
            }
        }

        impl Default for Metadata {
            fn default() -> Self {
                Self::new()
            }
        }

        impl Stored for Metadata {
            fn is_obsolete(&self) -> bool {
                self.hash.is_none()
            }
        }
    }

    pub mod by_sha256 {
        use crate::core::vault::{caching::storage::Stored, volume};

        #[derive(serde::Serialize, serde::Deserialize)]
        pub struct Metadata {
            paths: std::collections::HashSet<volume::VolumePath>,
        }

        impl Metadata {
            pub fn new() -> Self {
                Self {
                    paths: std::collections::HashSet::new(),
                }
            }

            pub fn add_path(&mut self, path: volume::VolumePath) {
                self.paths.insert(path);
            }

            pub fn paths(&self) -> impl std::iter::Iterator<Item = &volume::VolumePath> {
                self.paths.iter()
            }
        }

        impl Default for Metadata {
            fn default() -> Self {
                Self::new()
            }
        }

        impl Stored for Metadata {
            fn is_obsolete(&self) -> bool {
                self.paths.is_empty()
            }
        }
    }

    pub mod by_id {
        use crate::core::vault::caching::storage::Stored;

        #[derive(serde::Serialize, serde::Deserialize)]
        pub struct Metadata {
            title: Option<String>,
        }

        impl Metadata {
            pub fn new() -> Self {
                Self { title: None }
            }

            pub fn get_title(&self) -> Option<&String> {
                self.title.as_ref()
            }

            pub fn set_title(&mut self, title: String) {
                self.title = Some(title);
            }
        }

        impl Default for Metadata {
            fn default() -> Self {
                Self::new()
            }
        }

        impl Stored for Metadata {}
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct GlobalCacheSaveState {}

pub struct GlobalCache {
    base_path: std::path::PathBuf,
    by_id: storage::DataStorage<caches::by_id::Metadata>,
    by_path: storage::DataStorage<caches::by_path::Metadata>,
    by_sha256: storage::DataStorage<caches::by_sha256::Metadata>,
}

impl GlobalCache {
    pub fn new(base_path: std::path::PathBuf) -> Self {
        let by_id = base_path.join("by_id");
        let by_path = base_path.join("by_path");
        let by_sha256 = base_path.join("by_sha256");

        Self {
            base_path,
            by_id: storage::DataStorage::open(by_id, true).unwrap(),
            by_path: storage::DataStorage::open(by_path, true).unwrap(),
            by_sha256: storage::DataStorage::open(by_sha256, true).unwrap(),
        }
    }

    pub fn get_hash(&mut self, path: &volume::VolumePath) -> Option<hashing::Sha256> {
        self.by_path
            .access(path.as_hash(), |metadata| {
                metadata.get_hash().cloned()
            })
            .ok()
            .flatten()
    }

    fn report_hash(&mut self, hash: &hashing::Sha256, path: &volume::VolumePath) {
        self.by_sha256
            .modify(hash.clone(), |metadata| {
                metadata.add_path(path.clone());
            })
            .map_err(|_| {
                eprintln!("Failed to report hash for path: {:?}", path);
            })
            .ok();
    }

    fn report_hash_2(&mut self, hash: &hashing::Sha256, path: &volume::VolumePath) {
        self.by_path
            .modify(path.as_hash(), |metadata| {
                metadata.set_hash(hash.clone());
            })
            .map_err(|_| {
                eprintln!("Failed to report hash for path: {:?}", path);
            })
            .ok();
    }

    pub fn set_hash(&mut self, path: &volume::VolumePath, hash: hashing::Sha256) {
        self.report_hash(&hash, &path); // Tell the system in which file(s) the hash can be found
        self.report_hash_2(&hash, &path); // Tell the system the hash of the file
    }

    pub fn get_title(&mut self, id: &entity::Id) -> Option<String> {
        self.by_id
            .access(id.clone(), |metadata| metadata.get_title().cloned())
            .ok()
            .flatten()
    }

    pub fn set_title(&mut self, id: entity::Id, title: String) {
        self.by_id
            .modify(id.clone(), |metadata| {
                metadata.set_title(title);
            })
            .map_err(|_| {
                eprintln!("Failed to set title for ID: {:?}", id);
            })
            .ok();
    }

    pub fn save(&mut self) -> Result<(), std::io::Error> {
        let snapshot = GlobalCacheSaveState {};

        {
            let path = self.base_path.join("cache.json");
            let file = std::fs::File::create(&path)?;
            let writer = std::io::BufWriter::new(file);
            serde_json::to_writer(writer, &snapshot)?;
        }

        self.by_id
            .flush_cache()
            .map_err(|_| {
                eprintln!("Failed to flush metadata cache for by_id");
            })
            .ok();

        self.by_path
            .flush_cache()
            .map_err(|_| {
                eprintln!("Failed to flush metadata cache for by_path");
            })
            .ok();

        self.by_sha256
            .flush_cache()
            .map_err(|_| {
                eprintln!("Failed to flush metadata cache for by_sha256");
            })
            .ok();

        Ok(())
    }
}
