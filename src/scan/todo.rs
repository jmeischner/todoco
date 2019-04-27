use crate::SourceFile;

#[derive(Debug)]
pub struct Todo {
    pub text: String,
    pub file: SourceFile,
    pub line: usize,
    // Todo: Save next few lines for preview
}

impl Todo {
    pub fn new(text: String, file: SourceFile, line: usize) -> Todo {
        Todo {
            text: text,
            file: file,
            line: line,
        }
    }
}

impl PartialEq for Todo {
    fn eq(&self, other: &Todo) -> bool {
        self.text == other.text && self.file == other.file && self.line == other.line
    }
}
