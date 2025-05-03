
#[derive(Debug, Clone)]
pub struct UniversityCourseId {
    _id: String,
}

impl UniversityCourseId {
    pub fn new(id: String) -> Self {
        UniversityCourseId { _id: id }
    }
}
