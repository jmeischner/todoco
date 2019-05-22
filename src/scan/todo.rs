use crate::SourceFile;

#[derive(Debug)]
pub struct Todo {
    pub text: String,
    pub file: SourceFile,
    pub line: usize,
    pub tags: Vec<Tag>, // Todo: Save next few lines for preview
}

impl Todo {
    pub fn new(text: String, file: SourceFile, line: usize) -> Todo {
        Todo {
            text: text,
            file: file,
            line: line,
            tags: vec![],
        }
    }

    pub fn add_tag(&mut self, tag: Tag) {
        self.tags.push(tag);
    }
}

impl PartialEq for Todo {
    fn eq(&self, other: &Todo) -> bool {
        self.text == other.text && self.file == other.file && self.line == other.line
    }
}

#[derive(Debug)]
pub struct Tag {
    pub name: String,
    pub value: Option<String>,
}

impl Tag {
    pub fn new(name: String, value: Option<String>) -> Tag {
        Tag {
            name: name,
            value: value,
        }
    }
}

impl PartialEq for Tag {
    fn eq(&self, other: &Tag) -> bool {
        self.name == other.name && self.value == other.value
    }
}
