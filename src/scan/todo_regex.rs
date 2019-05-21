use regex::{Regex, RegexSet};

pub struct TodoRegexer {
    has_todo: Regex,
    todo_patterns: RegexSet,
}

impl TodoRegexer {
    const HAS_TODO: &'static str = r"(?i)[//|/\*|#]\s*todo[:\s*|\s+](?P<todo>.*)$";
    const TODO_PATTERNS: [&'static str; 2] = [
        r"(?i)[//|/\*|#]\s*todo[:\s*|\s+](?P<todo>.*)$",
        r"(?i)[//|/\*|#]\s*todo[:\s*|\s+](?P<todo>.*)@(?P<tag>.*)\((?P<tagvalue>.*)\)$",
    ];

    pub fn new() -> TodoRegexer {
        TodoRegexer {
            has_todo: Regex::new(TodoRegexer::HAS_TODO).unwrap(),
            todo_patterns: RegexSet::new(&TodoRegexer::TODO_PATTERNS).unwrap(),
        }
    }

    pub fn is_todo(&self, line: &str) -> bool {
        self.has_todo.is_match(line)
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
}
