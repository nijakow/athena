use super::io::resource;


pub mod id;

pub mod file;
pub mod zettel;


pub type Id = id::Id;

pub enum Entity {
    File(file::File),
    Zettel(zettel::Zettel),
}

impl Entity {
    pub fn from_resource(resource: resource::Resource) -> Result<Self, Box<dyn std::error::Error>> {
        let metadata = resource.metadata();

        match metadata.resource_type {
            Some(resource::Type::Athena) => {
                let split_json = resource.read_to_split_json()?;
                let zettel = zettel::Zettel::from_split_json(&split_json).map_err(|_| "Failed to parse Zettel")?;

                Ok(Entity::Zettel(zettel))
            }
            Some(resource::Type::Obsidian) => {
                let markdown = resource.read_to_obsidian_markdown()?;
                let zettel = zettel::Zettel::from_obsidian_markdown(&markdown).map_err(|_| "Failed to parse Zettel")?;

                Ok(Entity::Zettel(zettel))
            }
            Some(resource::Type::Pdf) => Err("PDF not supported".into()),
            None => Err("Unknown resource type".into()),
        }
    }
}
