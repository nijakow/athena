
#[derive(Debug, Clone)]
pub struct UniversityCourseId {
    id: String,
}

impl UniversityCourseId {
    pub fn new(id: String) -> Self {
        UniversityCourseId { id }
    }
}
