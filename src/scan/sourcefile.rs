#[derive(Debug, Clone)]
pub struct SourceFile {
    pub name: String,
    pub path: String,
}

impl SourceFile {
    pub fn new(name: String, path: String) -> SourceFile {
        SourceFile {
            name: name,
            path: path,
        }
    }
}

impl PartialEq for SourceFile {
    fn eq(&self, other: &SourceFile) -> bool {
        self.name == other.name && self.path == other.path
    }
}
