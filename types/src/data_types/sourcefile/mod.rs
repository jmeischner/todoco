use serde::{Deserialize, Serialize};

#[derive(Hash, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct SourceFile {
    pub name: String,
    pub path: String,
}

impl SourceFile {
    pub fn new(name: &str, path: &str) -> SourceFile {
        SourceFile {
            name: name.to_string(),
            path: path.to_string(),
        }
    }
}

impl PartialEq for SourceFile {
    fn eq(&self, other: &SourceFile) -> bool {
        self.name == other.name && self.path == other.path
    }
}
