use crate::List;

#[derive(Debug)]
pub struct Project {
    pub name: String,
    pub lists: Vec<List>,
}

impl Project {
    pub fn new(name: String, lists: Vec<List>) -> Project {
        Project {
            name: name,
            lists: lists,
        }
    }
}
