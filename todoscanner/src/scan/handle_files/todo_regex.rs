use types::Tag;
use regex::Regex;

pub struct TodoRegexer {
    has_todo: Regex,
    has_tag: Regex,
    todo_pattern: Regex,
    tag_pattern: Regex,
}

impl TodoRegexer {
    const HAS_TODO: &'static str = r"(?i)[//|/\*|#]\s*todo";
    const HAS_TAG: &'static str = r".*@[^\s.*]";
    const TODO_PATTERN: &'static str = r"(?i)[//|/\*|#]\s*todo[:\s*|\s+](?P<todo>.*)$";
    const TAG_PATTERN: &'static str = r"(?i)(?P<text>.*)@(?P<tag>[^\(\)\s]*)(\((?P<value>.*)\))?$";

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

    pub fn extract_text<'a>(&self, line: &'a str) -> Option<&'a str> {
        if let Some(cap) = &self.todo_pattern.captures(&line) {
            if let Some(text) = cap.name("todo") {
                return Some(text.as_str().trim());
            }
        }

        None
    }

    pub fn extract_tags_from_text<'a>(&self, todo: &'a str) -> (&'a str, Vec<Tag>) {
        let mut tags: Vec<Tag> = vec![];
        let mut todo = todo;

        loop {
            if let (text, Some(tag)) = self.get_tag(&todo) {
                todo = text;
                tags.push(tag);
            } else {
                break;
            }
        }

        (todo, tags)
    }

    fn contains_tag(&self, todo: &str) -> bool {
        self.has_tag.is_match(todo)
    }

    fn get_tag<'a>(&self, todo: &'a str) -> (&'a str, Option<Tag>) {
        if self.contains_tag(&todo) {
            if let Some(caps) = self.tag_pattern.captures(todo) {
                if let Some(text) = caps.name("text") {
                    if let Some(tag) = caps.name("tag") {
                        if let Some(value) = caps.name("value") {
                            return (
                                text.as_str().trim_end(),
                                Some(Tag::new(
                                    tag.as_str(),
                                    Some(String::from(value.as_str())),
                                )),
                            );
                        } else {
                            return (
                                text.as_str().trim_end(),
                                Some(Tag::new(tag.as_str(), None)),
                            );
                        }
                    }
                }
            };
        };

        (todo, None)
    }
}

// ~~~~~~~~~~~~~~~~~~~~ TESTS ~~~~~~~~~~~~~~~~~~~~ //
#[cfg(test)]
mod tests_todo_regexer {
    use super::TodoRegexer;
    use types::Tag;

    fn build_todo_and_tag(text: &str) -> (&str, Option<Tag>) {
        let tr = TodoRegexer::new();
        tr.get_tag(text)
    }

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
        assert!(tr.contains_tag(line));
    }

    #[test]
    fn has_no_tag() {
        let tr = TodoRegexer::new();
        let line = "blabla @ blu bla bli";
        assert!(!tr.contains_tag(line));
    }

    #[test]
    fn get_single_tag() {
        let (text, tag) = build_todo_and_tag("blabla @bli(blu)");
        let tag = tag.unwrap();
        assert_eq!(text, String::from("blabla"));
        assert_eq!(tag.name, "bli");
        assert_eq!(tag.value.unwrap(), "blu");
    }

    #[test]
    fn get_single_tag_without_value() {
        let (text, tag) = build_todo_and_tag("blabla @bli");
        let tag = tag.unwrap();
        assert_eq!(text, String::from("blabla"));
        assert_eq!(tag.name, "bli");
        assert_eq!(tag.value, None);
    }

    #[test]
    fn get_single_tag_with_empty_value() {
        let (text, tag) = build_todo_and_tag("blabla @bli()");
        let tag = tag.unwrap();
        assert_eq!(text, String::from("blabla"));
        assert_eq!(tag.name, "bli");
        assert_eq!(tag.value, Some(String::from("")));
    }

    #[test]
    fn get_complicated_tag() {
        let (text, tag) = build_todo_and_tag("blabla @bli-blu-1(12-12-we)");
        let tag = tag.unwrap();
        assert_eq!(text, String::from("blabla"));
        assert_eq!(tag.name, "bli-blu-1");
        assert_eq!(tag.value, Some(String::from("12-12-we")));
    }

    #[test]
    fn do_not_register_tag_in_the_middle_of_test() {
        let (text, tag) = build_todo_and_tag("blabla @bli-blu 1(12-12-we)");
        let tag = tag;
        assert_eq!(text, String::from("blabla @bli-blu 1(12-12-we)"));
        assert_eq!(tag, None);
    }

    #[test]
    fn get_last_tag_if_multiple_exist() {
        let (text, tag) = build_todo_and_tag("blabla @bli(blu) @blubb(blibb)");
        let tag = tag.unwrap();
        assert_eq!(text, String::from("blabla @bli(blu)"));
        assert_eq!(tag.name, "blubb");
        assert_eq!(tag.value, Some(String::from("blibb")));
    }

    #[test]
    fn get_multiple_tags_from_text() {
        let tr = TodoRegexer::new();
        let todo = "blabla @bli(blo) @blubb(blibb)";
        let (text, tags) = tr.extract_tags_from_text(todo);
        assert_eq!(text, "blabla");
        assert_eq!(tags.len(), 2);
        assert_eq!(
            tags[0],
            Tag::new("blubb", Some(String::from("blibb")))
        );
        assert_eq!(
            tags[1],
            Tag::new("bli", Some(String::from("blo")))
        );
    }

}
