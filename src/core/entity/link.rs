
pub mod reference {
    use crate::core::entity;

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum Reference {
        Entity(entity::Id),
        Url(url::Url),
    }
}
