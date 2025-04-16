pub mod embedding;
pub mod hashing;

pub fn split_metadata_from_content(content: String) -> Option<(String, String)> {
    let first_delimiter = content.find("---")?;
    let second_delimiter = content[first_delimiter + 3..].find("---")?;

    let header = &content[first_delimiter + 3..first_delimiter + 3 + second_delimiter];
    let body = &content[first_delimiter + 3 + second_delimiter + 3..];

    Some((header.to_string(), body.to_string()))
}
