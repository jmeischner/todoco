use crate::SourceFile;

#[derive(Debug)]
pub struct Todo {
    pub text: String,
    pub file: SourceFile,
    pub line: usize,
    pub tags: Vec<Tag>, // Todo: Save next few lines for preview
}

impl Todo {
    pub fn new(text: String, file: SourceFile, line: usize, tags: Vec<Tag>) -> Todo {
        Todo {
            text: text,
            file: file,
            line: line,
            tags: tags,
        }
    }

    pub fn set_tags(&mut self, tags: Vec<Tag>) -> &mut Todo {
        self.tags = tags;
        self
    }
}

impl PartialEq for Todo {
    fn eq(&self, other: &Todo) -> bool {
        self.text == other.text
            && self.file == other.file
            && self.line == other.line
            && self.tags == other.tags
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
