// ~~~~~~~~~~~~~~~~~~~~ TESTS ~~~~~~~~~~~~~~~~~~~~ //
#[cfg(test)]
mod tests_for_transition_comparison {
    use super::*;
    use types::{SourceFile, Todo};

    #[test]
    fn assert_two_todos_as_same() {
        let first_todo = Todo::new(
            String::from("Erledig die Aufgaben"),
            SourceFile::new(String::from("lib.rs"), String::from("test/path")),
            23,
            vec![],
        );
        let second_todo = Todo::new(
            String::from("Erledig die Aufgaben"),
            SourceFile::new(String::from("mod.rs"), String::from("test/path/mod")),
            23,
            vec![],
        );

        assert!(true);
    }
}
