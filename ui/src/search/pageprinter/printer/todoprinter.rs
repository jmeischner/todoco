use super::ItemPrinter;
use crate::helper;
use console::{style, Term};
use std::io::Result as IOResult;
use std::path::Path;
use types::Todo;

#[derive(Clone)]
pub struct TodoPrinter {}

impl TodoPrinter {
    fn print_todo(&self, term: &Term, todo: &Todo) -> IOResult<()> {
        let list_marker = format!("{}", style("◗").green());
        let at = format!("{}", style("at").cyan());
        let mut todo_text = format!("{}{} {}", helper::tab(2), list_marker, todo.text);

        if todo.tags.len() > 0 {
            let tags_text = self.get_tag_str(todo);
            todo_text.push_str(&tags_text);
        }

        term.write_line(&todo_text)?;
        term.write_line(&format!(
            "{}{} {}:{}",
            helper::tab(3),
            at,
            Path::new(&todo.file.path)
                .join(&todo.file.name)
                .to_str()
                .unwrap_or("Could not display path"),
            todo.line
        ))
    }

    fn get_tag_str(&self, todo: &Todo) -> String {
        let mut tag_string = String::new();

        for tag in &todo.tags {
            tag_string.push_str(&format!(
                " {}{}",
                style("@").green(),
                style(&tag.name).cyan()
            ));

            if let Some(value) = &tag.value {
                tag_string.push_str(&format!("{}", style(format!("({})", value)).yellow()));
            }
        }

        tag_string
    }
}

impl ItemPrinter<Todo> for TodoPrinter {
    fn new() -> TodoPrinter {
        TodoPrinter {}
    }

    fn print_items(&self, term: &Term, items: &[Todo]) -> IOResult<()> {
        for todo in items {
            self.print_todo(term, todo)?;
        }

        Ok(())
    }

    fn get_item_height(&self) -> usize {
        2
    }
}

