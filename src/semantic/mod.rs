
pub mod knowledge;


pub enum InfoItem {
    Task,
    Tag,
    Link,
}


pub trait Scannable {
    fn extract_info_items(&self, text: &str) -> Vec<InfoItem>;
}
