use std::sync::RwLock;

use crate::volt::resource;
use serde::ser::SerializeStruct;

pub struct VolumeCache {
    pub cache_file_path: std::path::PathBuf,
    pub file_name_cache: std::collections::HashMap<String, std::path::PathBuf>,
    pub resource_cache: RwLock<resource::cache::ResourceCache>,
}

pub struct VolumeCacheSnapshot {
    pub file_name_cache: std::collections::HashMap<String, std::path::PathBuf>,
    pub resource_cache: resource::cache::ResourceCacheSnapshot,
}

impl crate::util::snapshotting::Snapshottable for VolumeCache {
    type Snapshot = VolumeCacheSnapshot;

    fn from_snapshot(&mut self, snapshot: Self::Snapshot) {
        self.file_name_cache = snapshot.file_name_cache;
        self.resource_cache.write().unwrap().from_snapshot(snapshot.resource_cache);
    }

    fn take_snapshot(&self) -> Self::Snapshot {
        VolumeCacheSnapshot {
            file_name_cache: self.file_name_cache.clone(),
            resource_cache: self.resource_cache.read().unwrap().take_snapshot(),
        }
    }
}

impl serde::Serialize for VolumeCacheSnapshot {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("CacheSnapshot", 2)?;
        state.serialize_field("file_name_cache", &self.file_name_cache)?;
        state.serialize_field("resource_cache", &self.resource_cache)?;
        state.end()
    }
}

impl<'de> serde::Deserialize<'de> for VolumeCacheSnapshot {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct CacheSnapshotHelper {
            file_name_cache: std::collections::HashMap<String, std::path::PathBuf>,
            resource_cache: resource::cache::ResourceCacheSnapshot,
        }

        let helper = CacheSnapshotHelper::deserialize(deserializer)?;
        Ok(VolumeCacheSnapshot {
            file_name_cache: helper.file_name_cache,
            resource_cache: helper.resource_cache,
        })
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct VolumesCacheSnapshot {
    pub volumes: std::collections::HashMap<crate::volt::volume::VolumeId, VolumeCacheSnapshot>,
}
