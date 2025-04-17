
pub enum UserDirectory {
    Home,
    Documents,
}

pub enum DirectoryPurpose {
    UserDirectory(UserDirectory),
    UniversityCourse(crate::semantic::knowledge::UniversityCourseId),
}
