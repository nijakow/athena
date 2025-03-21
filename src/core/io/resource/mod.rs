use crate::core::entity;

#[derive(Debug, Clone, Copy, enum_iterator::Sequence)]
pub enum ZettelType {
    Athena,
    Obsidian,
}

#[derive(Debug, Clone, Copy, enum_iterator::Sequence)]
pub enum DocumentType {
    PlainText,
    Pdf,
}

#[derive(Debug, Clone, Copy, enum_iterator::Sequence)]
pub enum ImageType {
    Png,
    Jpg,
    Webp,
    Gif,
    Svg,
    Bmp,
}

#[derive(Debug, Clone, Copy, enum_iterator::Sequence)]
pub enum VideoType {
    Mp4,
    Webm,
    Ogg,
}

#[derive(Debug, Clone, Copy, enum_iterator::Sequence)]
pub enum Type {
    Zettel(ZettelType),
    Document(DocumentType),
    Image(ImageType),
    Video(VideoType),
    Unknown,
}

impl Type {

    pub fn to_extensions(&self) -> Vec<&'static str> {
        match self {
            Type::Zettel(ZettelType::Athena) => vec!["zson"],
            Type::Zettel(ZettelType::Obsidian) => vec!["md"],
            Type::Document(DocumentType::PlainText) => vec!["txt"],
            Type::Document(DocumentType::Pdf) => vec!["pdf"],
            Type::Image(ImageType::Png) => vec!["png"],
            Type::Image(ImageType::Jpg) => vec!["jpg", "jpeg"],
            Type::Image(ImageType::Webp) => vec!["webp"],
            Type::Image(ImageType::Gif) => vec!["gif"],
            Type::Image(ImageType::Svg) => vec!["svg"],
            Type::Image(ImageType::Bmp) => vec!["bmp"],
            Type::Video(VideoType::Mp4) => vec!["mp4"],
            Type::Video(VideoType::Webm) => vec!["webm"],
            Type::Video(VideoType::Ogg) => vec!["ogg"],
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
            Type::Zettel(ZettelType::Athena) => "application/json",
            Type::Zettel(ZettelType::Obsidian) => "text/markdown",
            Type::Document(DocumentType::PlainText) => "text/plain",
            Type::Document(DocumentType::Pdf) => "application/pdf",
            Type::Image(ImageType::Png) => "image/png",
            Type::Image(ImageType::Jpg) => "image/jpeg",
            Type::Image(ImageType::Webp) => "image/webp",
            Type::Image(ImageType::Gif) => "image/gif",
            Type::Image(ImageType::Svg) => "image/svg+xml",
            Type::Image(ImageType::Bmp) => "image/bmp",
            Type::Video(VideoType::Mp4) => "video/mp4",
            Type::Video(VideoType::Webm) => "video/webm",
            Type::Video(VideoType::Ogg) => "video/ogg",
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

pub struct SplitJson {
    pub header: serde_json::Value,
    pub body: serde_json::Value,
}

impl SplitJson {
    pub fn parse<S: ToString>(content: S) -> Result<SplitJson, Box<dyn std::error::Error>> {
        /*
         * The SplitJson format is a file that begins with `---`, followed by a JSON object, followed by `---`, followed by another JSON object.
         */

        let content = content.to_string();

        // Find the position of the first `---`
        let first_delimiter = content
            .find("---")
            .ok_or("Failed to find first delimiter")?;
        let second_delimiter = content[first_delimiter + 3..]
            .find("---")
            .ok_or("Failed to find second delimiter")?;

        // Get the header and body JSON objects
        let header = &content[first_delimiter + 3..first_delimiter + 3 + second_delimiter];
        let body = &content[first_delimiter + 3 + second_delimiter + 3..];

        // Parse the JSON objects
        let header = serde_json::from_str(header)?;
        let body = serde_json::from_str(body)?;

        Ok(SplitJson { header, body })
    }
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

    pub fn read_to_split_json(&self) -> Result<SplitJson, Box<dyn std::error::Error>> {
        // Get the content from the bytes as UTF-8
        let content = std::fs::read_to_string(&self.path)?;

        // Parse the content into a SplitJson object
        SplitJson::parse(content)
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

    fn get_hash(&self, path: &std::path::Path) -> Option<&crate::util::hashing::Sha256> {
        // TODO: Check if the file has been modified since the hash was calculated
        self.hashes.get(path)
    }

    fn set_hash(&mut self, path: std::path::PathBuf, hash: crate::util::hashing::Sha256) {
        self.hashes.insert(path, hash);
    }
}
