use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub use_gitignore: bool,
}

impl Project {
    pub fn new(name: String, use_gitignore: bool) -> Project {
        Project {
            name: name,
            use_gitignore: use_gitignore,
        }
    }
}
