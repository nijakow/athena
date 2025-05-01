use crate::core::entity::zettel::document::node::reference::ReferenceTarget;


pub mod knowledge;


#[derive(Debug, Clone)]
pub enum InfoItem {
    Task,
    Tag(String),
    Link(ReferenceTarget),
}


pub trait Scannable {
    fn iterate_info_items<F: FnMut(InfoItem)>(&self, func: &mut F);
}
