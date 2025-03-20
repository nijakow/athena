use crate::core::io::resource::Type;

pub struct File {
    file_type: Type,
    content: Vec<u8>,
}

impl File {
    pub fn new(file_type: Type, content: Vec<u8>) -> Self {
        Self { file_type, content }
    }

    pub fn mime_type(&self) -> &str {
        self.file_type
            .mime_type()
            .unwrap_or("application/octet-stream")
    }

    pub fn content(&self) -> &[u8] {
        &self.content
    }

    pub fn extract_content(self) -> Vec<u8> {
        self.content
    }
}
