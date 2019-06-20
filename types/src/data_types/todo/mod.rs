use super::sourcefile::SourceFile;
use super::tag::Tag;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// const EQUALITY_THRESHOLD: f32 = 0.8;

#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub struct Todo {
    id: Uuid,
    pub text: String,
    pub file: SourceFile,
    pub line: usize,
    pub tags: Vec<Tag>, // Todo: Save next few lines for preview
}

impl Todo {
    pub fn new(text: String, file: SourceFile, line: usize, tags: Vec<Tag>) -> Todo {
        Todo {
            id: Uuid::new_v4(),
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

    pub fn is_similar_to(&self, other: &Todo) -> bool {
        self.text == other.text
            && self.file == other.file
            && self.line == other.line
            && self.tags == other.tags
    }
}

impl PartialEq for Todo {
    fn eq(&self, other: &Todo) -> bool {
        self.id == other.id
    }
}
