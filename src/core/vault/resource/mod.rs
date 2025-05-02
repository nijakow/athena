use super::{caching, volume};


pub mod file;
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

    pub fn from_url(url: &url::Url) -> Option<Self> {
        let path = url.path();
        let extension = std::path::Path::new(path)
            .extension()
            .and_then(|e| e.to_str());

        Self::from_extension(extension?)
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


pub trait ResourceInterface {
    fn open_for_reading(&self, path: &volume::VolumePath) -> Result<Box<dyn std::io::Read>, std::io::Error>;
}


pub struct Metadata {
    pub resource_type: Option<Type>,
}

#[derive(Debug)]
pub struct Resource {
    path: volume::VolumePath,
}

impl Resource {
    pub fn from_path(path: volume::VolumePath) -> Self {
        Self { path }
    }

    pub fn metadata(&self) -> Metadata {
        let extension = self.path.path().extension().and_then(|e| e.to_str());
        let resource_type = extension.and_then(|e| Type::from_extension(e));

        Metadata { resource_type }
    }

    pub fn volume_path(&self) -> &volume::VolumePath {
        &self.path
    }

    pub fn content_hash(&self, resource_interface: &dyn ResourceInterface, cache: &mut caching::GlobalCache) -> Option<crate::util::hashing::Sha256> {
        if self.is_usually_hash_addressable() {
            match cache.get_hash(&self.path) {
                Some(hash) => return Some(hash.clone()),
                None => {
                    let content = self.read_to_bytes(resource_interface).ok()?;
                    let hash = crate::util::hashing::Sha256::hash_bytes(&content);
                    cache.set_hash(&self.path, hash.clone());
                    Some(hash)
                }
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

    pub fn open_for_reading(&self, resource_interface: &dyn ResourceInterface) -> Result<Box<dyn std::io::Read>, std::io::Error> {
        // TODO, FIXME, XXX: Actually ask the volume! Don't ignore the volume ID!
        resource_interface.open_for_reading(&self.path)
    }

    pub fn read_to_bytes(&self, resource_interface: &dyn ResourceInterface) -> Result<Vec<u8>, std::io::Error> {
        self.open_for_reading(resource_interface)
            .and_then(|mut reader| {
                let mut buffer = Vec::new();
                reader.read_to_end(&mut buffer)?;
                Ok(buffer)
            })
    }

    pub fn read_to_string(&self, resource_interface: &dyn ResourceInterface) -> Result<String, std::io::Error> {
        self.open_for_reading(resource_interface)
            .and_then(|mut reader| {
                let mut buffer = String::new();
                reader.read_to_string(&mut buffer)?;
                Ok(buffer)
            })
    }

    pub fn read_content(&self, resource_interface: &dyn ResourceInterface) -> Result<file::FileContent, std::io::Error> {
        // TODO, FIXME, XXX: Actually ask the volume! Don't ignore the volume ID!
        let title = self
            .path
            .path()
            .file_stem()
            .and_then(|s| s.to_str())
            .map(|s| s.to_string());
        let content = self.read_to_bytes(resource_interface)?;
        let file_type = self.metadata().resource_type.unwrap_or(Type::Unknown);

        Ok(file::FileContent::new(file_type, title, content))
    }

    pub fn parse<T, E>(&self, parser_func: fn(file::FileContent) -> Result<T, E>, resource_interface: &dyn ResourceInterface) -> Result<T, ParseError<E>>
    {
        let content = self.read_content(resource_interface).map_err(|e| ParseError::Io(e))?;
        parser_func(content).map_err(|e| ParseError::Parse(e))
    }
}

pub enum ParseError<E> {
    Io(std::io::Error),
    Parse(E),
}
