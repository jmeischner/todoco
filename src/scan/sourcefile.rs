use std::ffi::OsString;

#[derive(Debug, Clone)]
pub struct SourceFile {
    pub name: OsString,
    pub path: OsString,
}

impl PartialEq for SourceFile {
    fn eq(&self, other: &SourceFile) -> bool {
        self.name == other.name && self.path == other.path
    }
}
