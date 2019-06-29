use super::itemprinter::ItemPrinter;
use crate::helper;
use types::Todo;
use std::io::Result as IOResult;
use console::{Term, style};

#[derive(Clone)]
pub struct TodoPrinter {}

impl TodoPrinter {
    fn print_todo(&self, term: &Term, todo: &Todo) -> IOResult<()> {
        let list_marker = format!("{}", style("◗").green());

        term.write_line(&format!("{}{} {}", helper::tab(2), list_marker, todo.text))
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
}




