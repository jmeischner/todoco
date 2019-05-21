use crate::{Tag, Todo};
use regex::{Regex, RegexSet};

pub struct TodoRegexer {
    has_todo: Regex,
    has_tag: Regex,
    todo_pattern: Regex,
    tag_pattern: Regex,
}

impl TodoRegexer {
    const HAS_TODO: &'static str = r"(?i)[//|/\*|#]\s*todo[:\s*|\s+](?P<todo>.*)$";
    const HAS_TAG: &'static str = r".*@[^\s.*]";
    const TODO_PATTERN: &'static str = r"(?i)[//|/\*|#]\s*todo[:\s*|\s+](?P<todo>.*)$";
    // Todo: Make Value Optional but match it if its there
    const TAG_PATTERN: &'static str = r"(?i).*@(?P<tag>.*)\((?P<value>.*)\)$";

    pub fn new() -> TodoRegexer {
        TodoRegexer {
            has_todo: Regex::new(TodoRegexer::HAS_TODO).unwrap(),
            has_tag: Regex::new(TodoRegexer::HAS_TAG).unwrap(),
            todo_pattern: Regex::new(TodoRegexer::TODO_PATTERN).unwrap(),
            tag_pattern: Regex::new(TodoRegexer::TAG_PATTERN).unwrap(),
        }
    }

    pub fn is_todo(&self, line: &str) -> bool {
        self.has_todo.is_match(line)
    }

    fn has_tag(&self, todo: &str) -> bool {
        self.has_tag.is_match(todo)
    }

    pub fn get_tag(&self, todo: &str) -> Option<Tag> {
        if let Some(caps) = &self.tag_pattern.captures(todo) {
            if let Some(tag) = caps.name("tag") {
                if let Some(value) = caps.name("value") {
                    Some(Tag::new(
                        String::from(tag.as_str()),
                        Some(String::from(value.as_str())),
                    ))
                } else {
                    Some(Tag::new(String::from(tag.as_str()), None))
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}

// ~~~~~~~~~~~~~~~~~~~~ TESTS ~~~~~~~~~~~~~~~~~~~~ //
#[cfg(test)]
mod tests {
    use super::TodoRegexer;

    #[test]
    fn check_if_line_is_todo() {
        let todo_r = TodoRegexer::new();
        let line = "  // todo: my test line";
        assert!(todo_r.is_todo(line));
    }

    #[test]
    fn todo_has_tag() {
        let tr = TodoRegexer::new();
        let line = "blabla @bla(bli)";
        assert!(tr.has_tag(line));
    }

    #[test]
    fn has_no_tag() {
        let tr = TodoRegexer::new();
        let line = "blabla @ blu bla bli";
        assert!(!tr.has_tag(line));
    }

    #[test]
    fn get_single_tag() {
        let tr = TodoRegexer::new();
        let todo = "blabla @bli(blu)";
        let tag = tr.get_tag(todo).unwrap();
        assert_eq!(tag.name, "bli");
        assert_eq!(tag.value.unwrap(), "blu");
    }

    #[test]
    fn get_single_tag_without_value() {
        let tr = TodoRegexer::new();
        let todo = "blabla @bli";
        let tag = tr.get_tag(todo).unwrap();
        assert_eq!(tag.name, "bli");
        assert_eq!(tag.value, None);
    }

    #[test]
    fn get_single_tag_with_empty_value() {
        let tr = TodoRegexer::new();
        let todo = "blabla @bli()";
        let tag = tr.get_tag(todo).unwrap();
        assert_eq!(tag.name, "bli()");
        assert_eq!(tag.value, Some(String::from("")));
    }
}
