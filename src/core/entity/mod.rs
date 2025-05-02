use crate::semantic;
use crate::core::vault;


pub mod id;

pub mod zettel;

pub type Id = id::Id;

pub enum Entity {
    File(vault::resource::file::FileContent), // TODO: Rework this
    Zettel(zettel::Zettel),
}

impl Entity {
    pub fn from_resource(resource: vault::resource::Resource, resource_interface: &dyn vault::resource::ResourceInterface) -> Result<Self, ()> {
        let metadata = resource.metadata();

        match metadata.resource_type {
            Some(vault::resource::Type::Zettel(vault::resource::types::ZettelType::Obsidian)) => {
                match resource.parse(crate::formats::markdown::parse_obsidian_markdown, resource_interface) {
                    Ok(document) => {
                        let zettel = zettel::Zettel::from_obsidian_markdown(document)?;
                        Ok(Entity::Zettel(zettel))
                    }
                    Err(_) => Err(()),
                }
            }
            Some(_) => {
                // TODO: Check file size and decide if it's too big to read into memory
                resource.read_content(resource_interface).map(|content| Entity::File(content)).map_err(|_| ())
            }
            None => Err(()),
        }
    }
}

impl semantic::Scannable for Entity {
    fn iterate_info_items<F: FnMut(semantic::InfoItem)>(&self, func: &mut F) {
        match self {
            Entity::File(_) => {}
            Entity::Zettel(zettel) => zettel.iterate_info_items(func),
        }
    }
}
