use crate::volt;


pub mod id;

pub mod zettel;

pub type Id = id::Id;

pub enum Entity {
    File(volt::resource::file::FileContent), // TODO: Rework this
    Zettel(zettel::Zettel),
}

impl Entity {
    pub fn from_resource(resource: volt::resource::Resource) -> Result<Self, ()> {
        let metadata = resource.metadata();

        match metadata.resource_type {
            Some(volt::resource::Type::Zettel(volt::resource::types::ZettelType::Obsidian)) => {
                match resource.parse(crate::formats::markdown::parse_obsidian_markdown) {
                    Ok(document) => {
                        let zettel = zettel::Zettel::from_obsidian_markdown(document)?;
                        Ok(Entity::Zettel(zettel))
                    }
                    Err(_) => Err(()),
                }
            }
            Some(_) => {
                // TODO: Check file size and decide if it's too big to read into memory
                resource.read_content().map(|content| Entity::File(content)).map_err(|_| ())
            }
            None => Err(()),
        }
    }
}
