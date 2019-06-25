use crate::Todo;

#[derive(Debug)]
pub enum FilterMatch {
    All(Vec<Todo>),
    Tags(Vec<Todo>),
    Files(Vec<Todo>),
    Text(Vec<Todo>),
    None,
}