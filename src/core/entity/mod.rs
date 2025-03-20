
pub mod id;

pub mod file;
pub mod zettel;


pub type Id = id::Id;

pub enum Entity {
    File(file::File),
    Zettel(zettel::Zettel),
}
