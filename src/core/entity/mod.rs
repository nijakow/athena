
pub mod id;

pub mod file;
pub mod zettel;


pub enum Entity {
    File(file::File),
    Zettel(zettel::Zettel),
}
