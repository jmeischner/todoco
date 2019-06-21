use super::sourcefile::SourceFile;
use super::tag::Tag;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::f64;
use strsim;
use uuid::Uuid;

// Todo: Research for best threshold value
const EQUALITY_THRESHOLD: f64 = 0.93;

#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub struct Todo {
    id: Uuid,
    pub is_active: bool,
    pub text: String,
    pub file: SourceFile,
    pub line: usize,
    pub tags: Vec<Tag>, // Todo: Save next few lines for preview
}

impl Todo {
    pub fn new(text: String, file: SourceFile, line: usize, tags: Vec<Tag>) -> Todo {
        Todo {
            id: Uuid::new_v4(),
            is_active: true,
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
        self.text == other.text && self.file == other.file && self.line == other.line
    }

    pub fn set_inactive(&mut self) {
        self.is_active = false;
    }

    pub fn match_in(&self, others: &Vec<Todo>) -> Option<Todo> {
        if let Some(matched) = others.iter().find(|todo| self.is_similar_to(todo)) {
            return Some(matched.clone());
        }

        let (metric, matched) = others
            .iter()
            .map(|todo| (todo.get_string_metric(&self), Some(todo)))
            .filter(|(metric, _)| metric > &EQUALITY_THRESHOLD)
            .fold(
                (0.0, None),
                |(m1, t1), (m2, t2)| match PartialOrd::partial_cmp(&m1, &m2) {
                    None => (f64::INFINITY, None),
                    Some(Ordering::Greater) => (m1, t1),
                    Some(_) => (m2, t2),
                },
            );

        println!("String Metric: {}", metric);

        if let Some(matched) = matched {
            return Some(matched.clone());
        }

        None
    }

    pub fn update_with(&self, other: &Todo) -> Todo {
        let mut result = other.clone();
        result.id = self.id;

        result
    }

    fn build_compare_string(&self) -> String {
        format!(
            "{}/{}:{} {}",
            self.file.path, self.file.name, self.line, self.text
        )
    }

    fn get_string_metric(&self, other: &Todo) -> f64 {
        strsim::jaro(&self.build_compare_string(), &other.build_compare_string())
    }
}

impl PartialEq for Todo {
    fn eq(&self, other: &Todo) -> bool {
        self.id == other.id
    }
}

// ~~~~~~~~~~~~~~~~~~~~ TESTS ~~~~~~~~~~~~~~~~~~~~ //
#[cfg(test)]
mod tests_todo {
    use super::EQUALITY_THRESHOLD;
    use crate::{SourceFile, Tag, Todo};

    #[test]
    fn match_two_nearly_similiar_todos() {
        let todo1 = Todo::new(
            "My Test Todo".to_string(),
            SourceFile::new("here.txt".to_string(), "please/look".to_string()),
            4,
            vec![Tag::new("hey".to_string(), None)],
        );

        let todo2 = Todo::new(
            "My second Test Todo".to_string(),
            SourceFile::new("here.txt".to_string(), "please/look".to_string()),
            4,
            vec![Tag::new("hey".to_string(), None)],
        );

        assert!(todo1.get_string_metric(&todo2) > EQUALITY_THRESHOLD);
    }

    #[test]
    fn not_match_two_not_so_similiar_todos() {
        let todo1 = Todo::new(
            "You have to do this".to_string(),
            SourceFile::new("here.txt".to_string(), "please/look".to_string()),
            4,
            vec![Tag::new("hey".to_string(), None)],
        );

        let todo2 = Todo::new(
            "Just Do".to_string(),
            SourceFile::new("here.txt".to_string(), "please/look".to_string()),
            4,
            vec![Tag::new("hey".to_string(), None)],
        );

        assert!(todo1.get_string_metric(&todo2) < EQUALITY_THRESHOLD);
    }

    #[test]
    fn not_match_two_similar_todos_in_different_files() {
        let todo1 = Todo::new(
            "My Test Todo".to_string(),
            SourceFile::new(
                "another file.txt".to_string(),
                "please/drink/water".to_string(),
            ),
            4,
            vec![Tag::new("hey".to_string(), None)],
        );

        let todo2 = Todo::new(
            "My Test Todo".to_string(),
            SourceFile::new("here.txt".to_string(), "please/look".to_string()),
            4,
            vec![Tag::new("hey".to_string(), None)],
        );

        assert!(todo1.get_string_metric(&todo2) < EQUALITY_THRESHOLD);
    }

    #[test]
    fn match_two_nearly_similiar_todos_in_renamed_files() {
        let todo1 = Todo::new(
            "My Test Todo".to_string(),
            SourceFile::new("here.txt".to_string(), "please/look".to_string()),
            4,
            vec![Tag::new("hey".to_string(), None)],
        );

        let todo2 = Todo::new(
            "My Test Todo".to_string(),
            SourceFile::new("away.txt".to_string(), "please/look".to_string()),
            4,
            vec![Tag::new("hey".to_string(), None)],
        );

        assert!(todo1.get_string_metric(&todo2) > EQUALITY_THRESHOLD);
    }
}
