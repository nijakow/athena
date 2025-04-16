use crate::core::entity;

pub mod types;


#[derive(Debug, Clone, Copy, enum_iterator::Sequence)]
pub enum Type {
    Zettel(types::ZettelType),
    Document(types::DocumentType),
    Image(types::ImageType),
    Audio(types::AudioType),
    Video(types::VideoType),
    Unknown,
}

impl Type {
    pub fn to_extensions(&self) -> Vec<&'static str> {
        match self {
            Type::Zettel(types::ZettelType::Athena) => vec!["zson"],
            Type::Zettel(types::ZettelType::Obsidian) => vec!["md"],
            Type::Document(types::DocumentType::PlainText) => vec!["txt"],
            Type::Document(types::DocumentType::Pdf) => vec!["pdf"],
            Type::Image(types::ImageType::Png) => vec!["png"],
            Type::Image(types::ImageType::Jpg) => vec!["jpg", "jpeg"],
            Type::Image(types::ImageType::Webp) => vec!["webp"],
            Type::Image(types::ImageType::Gif) => vec!["gif"],
            Type::Image(types::ImageType::Svg) => vec!["svg"],
            Type::Image(types::ImageType::Bmp) => vec!["bmp"],
            Type::Audio(types::AudioType::Mp3) => vec!["mp3"],
            Type::Audio(types::AudioType::Ogg) => vec!["ogg"],
            Type::Audio(types::AudioType::Wav) => vec!["wav"],
            Type::Video(types::VideoType::Mp4) => vec!["mp4"],
            Type::Video(types::VideoType::Webm) => vec!["webm"],
            Type::Video(types::VideoType::Ogg) => vec!["ogg"],
            Type::Unknown => vec![],
        }
    }

    pub fn to_extension(&self) -> Option<&'static str> {
        self.to_extensions().first().copied()
    }

    pub fn from_extension(extension: &str) -> Option<Self> {
        Self::map_extensions()
            .iter()
            .find(|(_, e)| e == &extension)
            .map(|(t, _)| t)
            .copied()
    }

    pub fn mime_type(&self) -> &'static str {
        match self {
            Type::Zettel(types::ZettelType::Athena) => "application/json",
            Type::Zettel(types::ZettelType::Obsidian) => "text/markdown",
            Type::Document(types::DocumentType::PlainText) => "text/plain",
            Type::Document(types::DocumentType::Pdf) => "application/pdf",
            Type::Image(types::ImageType::Png) => "image/png",
            Type::Image(types::ImageType::Jpg) => "image/jpeg",
            Type::Image(types::ImageType::Webp) => "image/webp",
            Type::Image(types::ImageType::Gif) => "image/gif",
            Type::Image(types::ImageType::Svg) => "image/svg+xml",
            Type::Image(types::ImageType::Bmp) => "image/bmp",
            Type::Audio(types::AudioType::Mp3) => "audio/mpeg",
            Type::Audio(types::AudioType::Ogg) => "audio/ogg",
            Type::Audio(types::AudioType::Wav) => "audio/wav",
            Type::Video(types::VideoType::Mp4) => "video/mp4",
            Type::Video(types::VideoType::Webm) => "video/webm",
            Type::Video(types::VideoType::Ogg) => "video/ogg",
            Type::Unknown => "application/octet-stream",
        }
    }

    pub fn is_usually_immutable(&self) -> bool {
        match self {
            Type::Zettel(_) => false,
            _ => true,
        }
    }

    pub fn all() -> Vec<Self> {
        enum_iterator::all::<Self>().collect()
    }

    pub fn map_extensions() -> Vec<(Self, &'static str)> {
        Self::all()
            .iter()
            .flat_map(|t| {
                let extensions = t.to_extensions();
                extensions.into_iter().map(move |e| (*t, e))
            })
            .collect::<Vec<_>>()
            .into_iter()
            .collect()
    }

    pub fn all_extensions() -> Vec<&'static str> {
        Self::all().iter().flat_map(|t| t.to_extensions()).collect()
    }
}


pub struct Metadata {
    pub resource_type: Option<Type>,
}

pub struct Resource {
    path: std::path::PathBuf,
}

impl Resource {
    pub fn from_path(path: std::path::PathBuf) -> Self {
        Self { path }
    }

    pub fn metadata(&self) -> Metadata {
        let extension = self.path.extension().and_then(|e| e.to_str());
        let resource_type = extension.and_then(|e| Type::from_extension(e));

        Metadata { resource_type }
    }

    pub fn path(&self) -> &std::path::Path {
        &self.path
    }

    pub fn content_hash(&self, cache: &mut ResourceCache) -> Option<crate::util::hashing::Sha256> {
        if self.is_usually_hash_addressable() {
            if let Some(cached_hash) = cache.get_hash(&self.path) {
                Some(cached_hash.clone())
            } else {
                println!("Calculating hash for {:?}", self.path);
                let content = self.read_to_bytes().ok()?;
                let hash = crate::util::hashing::Sha256::hash_bytes(&content);
                cache.set_hash(self.path.clone(), hash.clone());
                Some(hash)
            }
        } else {
            None
        }
    }

    pub fn is_usually_hash_addressable(&self) -> bool {
        self.metadata()
            .resource_type
            .map_or(false, |t| t.is_usually_immutable())
    }

    pub fn open_for_reading(&self) -> Result<Box<dyn std::io::Read>, std::io::Error> {
        std::fs::File::open(&self.path).map(|f| Box::new(f) as Box<dyn std::io::Read>)
    }

    pub fn read_to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        std::fs::read(&self.path)
    }

    pub fn read_to_string(&self) -> Result<String, std::io::Error> {
        std::fs::read_to_string(&self.path)
    }

    pub fn read_to_file(&self) -> Result<entity::file::File, std::io::Error> {
        let title = self
            .path
            .file_stem()
            .and_then(|s| s.to_str())
            .map(|s| s.to_string());
        let content = self.read_to_bytes()?;
        let file_type = self.metadata().resource_type.unwrap_or(Type::Unknown);

        Ok(entity::file::File::new(file_type, title, content))
    }

    pub fn read_to_obsidian_markdown(
        &self,
    ) -> Result<crate::formats::markdown::Document, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(&self.path)?;

        crate::formats::markdown::parser::parse_document(content)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }
}

pub struct ResourceCache {
    hashes: std::collections::HashMap<std::path::PathBuf, crate::util::hashing::Sha256>,
}

impl ResourceCache {
    pub fn new() -> Self {
        Self {
            hashes: std::collections::HashMap::new(),
        }
    }

    pub fn load_from_file(path: &std::path::Path) -> std::io::Result<Self> {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let hashes: std::collections::HashMap<std::path::PathBuf, crate::util::hashing::Sha256> =
            serde_json::from_reader(reader)?;

        Ok(Self { hashes })
    }

    pub fn save_to_file(&self, path: &std::path::Path) -> std::io::Result<()> {
        let json = serde_json::to_string(&self.hashes)?;

        {
            use std::io::Write;

            let mut file = std::fs::File::create(path)?;
            file.write_all(json.as_bytes())?;
        }

        Ok(())
    }

    fn get_hash(&self, path: &std::path::Path) -> Option<&crate::util::hashing::Sha256> {
        // TODO: Check if the file has been modified since the hash was calculated
        self.hashes.get(path)
    }

    fn set_hash(&mut self, path: std::path::PathBuf, hash: crate::util::hashing::Sha256) {
        self.hashes.insert(path, hash);
    }
}
