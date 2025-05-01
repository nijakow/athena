use std::sync::RwLock;

use crate::volt::{self, resource};

pub struct VolumeCache {
    pub file_name_cache: std::collections::HashMap<String, std::path::PathBuf>,
    pub resource_cache: RwLock<resource::cache::ResourceCache>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct VolumeCacheSnapshot {
    pub resource_cache: resource::cache::ResourceCacheSnapshot,
}

impl<'a> crate::util::snapshotting::Snapshottable<'a> for VolumeCache {
    type Snapshot = VolumeCacheSnapshot;
    type Parameter = &'a volt::volume::Volume;

    fn from_snapshot(&mut self, snapshot: Self::Snapshot) {
        self.resource_cache.write().unwrap().from_snapshot(snapshot.resource_cache);
    }

    fn take_snapshot(&self, parameter: Self::Parameter) -> Self::Snapshot {
        VolumeCacheSnapshot {
            resource_cache: self.resource_cache.read().unwrap().take_snapshot(parameter),
        }
    }
}


#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct VolumesCacheSnapshot {
    pub volumes: std::collections::HashMap<crate::volt::volume::VolumeId, VolumeCacheSnapshot>,
}

impl VolumesCacheSnapshot {

    pub fn load_from_file(path: &std::path::Path) -> std::io::Result<Self> {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let snapshot = serde_json::from_reader(reader)?;

        Ok(snapshot)
    }

    pub fn save_to_file(&self, path: &std::path::Path) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self)?;

        {
            use std::io::Write;

            let mut file = std::fs::File::create(path)?;
            file.write_all(json.as_bytes())?;
        }

        Ok(())
    }

}
