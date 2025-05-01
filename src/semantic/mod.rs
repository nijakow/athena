
pub mod knowledge;


pub enum InfoItem {
    Task,
    Tag(String),
    Link,
}


pub trait Scannable {
    fn iterate_info_items<F: FnMut(InfoItem)>(&self, func: &mut F);
}
