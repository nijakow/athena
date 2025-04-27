pub mod embedding;
pub mod hashing;
pub mod snapshotting;

pub fn split_metadata_from_content(content: String) -> (Option<String>, String) {
    let (header, body) = content
        .split_once("---")
        .and_then(|(_, rest)| rest.split_once("---"))
        .map_or((None, content.clone()), |(header, body)| {
            (Some(header.trim().to_string()), body.to_string())
        });

    (header, body)
}
