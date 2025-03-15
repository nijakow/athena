
pub struct Files {
    base: std::path::PathBuf,
}

impl Files {
    pub fn new(base: std::path::PathBuf) -> Self {
        Files { base }
    }

    pub fn file<S: ToString>(&self, name: S) -> std::path::PathBuf {
        self.base.join(name.to_string())
    }
}
