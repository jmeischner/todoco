use crate::SourceFile;

pub struct Todo {
    pub text: String,
    pub file: SourceFile,
    pub line: usize,
}
