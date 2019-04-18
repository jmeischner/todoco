pub struct Project {
    pub name: String,
    pub lists: Vec<List>,
}

pub struct List {
    pub name: String,
    pub todos: Vec<Todo>,
}

pub struct Todo {
    pub text: String,
    pub file: String,
    pub line: u32,
}