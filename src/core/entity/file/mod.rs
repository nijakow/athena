use crate::core::io::resource::Type;

pub struct File {
    file_type: Type,
    path: std::path::PathBuf,
}

impl File {
    pub fn new(file_type: Type, path: std::path::PathBuf) -> Self {
        Self { file_type, path }
    }

    pub fn mime_type(&self) -> &str {
        self.file_type
            .mime_type()
            .unwrap_or("application/octet-stream")
    }

    pub fn content(&self) -> Result<Vec<u8>, std::io::Error> {
        std::fs::read(&self.path)
    }
}
