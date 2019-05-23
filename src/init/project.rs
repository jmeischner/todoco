use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub name: String,
    pub use_gitignore: bool,
    pub use_ignore: bool,
    pub search_hidden: bool,
}

impl Project {
    pub fn new(name: String, use_gitignore: bool) -> Project {
        Project {
            name: name,
            use_gitignore: use_gitignore,
            use_ignore: false,
            search_hidden: false,
        }
    }
}
