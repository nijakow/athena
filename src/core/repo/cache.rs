use std::sync::RwLock;

use crate::core::io::resource;

pub struct RepositoryCache {
    pub cache_file_path: std::path::PathBuf,
    pub file_name_cache: std::collections::HashMap<String, std::path::PathBuf>,
    pub resource_cache: RwLock<resource::cache::ResourceCache>,
}
