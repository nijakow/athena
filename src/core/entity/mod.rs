use super::io::resource;

pub mod id;

pub mod file;
pub mod zettel;

pub type Id = id::Id;

pub enum Entity {
    File(file::FileContent),
    Zettel(zettel::Zettel),
}

impl Entity {
    pub fn from_resource(resource: resource::Resource) -> Result<Self, Box<dyn std::error::Error>> {
        let metadata = resource.metadata();

        match metadata.resource_type {
            Some(resource::Type::Zettel(resource::types::ZettelType::Obsidian)) => {
                let document = resource.read_to_obsidian_markdown()?;
                let zettel = zettel::Zettel::from_obsidian_markdown(document)
                    .map_err(|_| "Failed to parse Zettel")?;

                Ok(Entity::Zettel(zettel))
            }
            Some(_) => {
                // TODO: Check file size and decide if it's too big to read into memory
                Ok(Entity::File(resource.read_content()?))
            }
            None => Err("Unknown resource type".into()),
        }
    }
}
