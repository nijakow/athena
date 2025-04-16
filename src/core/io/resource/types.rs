
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
pub enum AudioType {
    Mp3,
    Ogg,
    Wav,
}

#[derive(Debug, Clone, Copy, enum_iterator::Sequence)]
pub enum VideoType {
    Mp4,
    Webm,
    Ogg,
}

