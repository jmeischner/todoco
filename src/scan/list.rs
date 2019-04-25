use crate::Todo;

#[derive(Debug)]
pub struct List {
    pub name: String,
    pub todos: Vec<Todo>,
}

impl List {
    pub fn new(name: String, todos: Vec<Todo>) -> List {
        List {
            name: name,
            todos: todos,
        }
    }
}
