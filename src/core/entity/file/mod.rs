
pub struct File {
    path: std::path::PathBuf,
}

impl File {
    pub fn new(path: std::path::PathBuf) -> Self {
        Self { path }
    }
}
