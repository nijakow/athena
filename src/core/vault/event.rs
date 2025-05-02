
#[derive(Debug)]
pub enum FileEvent {
    Created,
    Modified,
    Deleted,
}

#[derive(Debug)]
pub enum VaultEvent {
    File(FileEvent),
}
