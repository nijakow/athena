use super::Type;

pub struct FileMetadata {
    file_type: Type,
    title: Option<String>,
}

impl FileMetadata {
    pub fn new(file_type: Type, title: Option<String>) -> Self {
        Self { file_type, title }
    }

    pub fn file_type(&self) -> Type {
        self.file_type
    }

    pub fn mime_type(&self) -> &str {
        self.file_type.mime_type()
    }

    pub fn title(&self) -> Option<String> {
        self.title.clone()
    }
}


pub struct FileContent {
    metadata: FileMetadata,
    content: Vec<u8>,
}

impl FileContent {
    pub fn new(file_type: Type, title: Option<String>, content: Vec<u8>) -> Self {
        Self {
            metadata: FileMetadata::new(file_type, title),
            content,
        }
    }

    pub fn metadata(&self) -> &FileMetadata {
        &self.metadata
    }

    pub fn content(&self) -> &[u8] {
        &self.content
    }

    pub fn content_as_string(&self) -> Option<String> {
        String::from_utf8(self.content.clone()).ok()
    }

    pub fn extract_content(self) -> Vec<u8> {
        self.content
    }

    pub fn extract_content_as_string(self) -> Option<String> {
        String::from_utf8(self.content).ok()
    }
}
