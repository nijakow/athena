use crate::core::entity::link::reference::Reference;


pub mod knowledge;


#[derive(Debug, Clone)]
pub enum InfoItem {
    Task,
    Tag(String),
    Link(Reference),
}


pub trait Scannable {
    fn iterate_info_items<F: FnMut(InfoItem)>(&self, func: &mut F);
}
