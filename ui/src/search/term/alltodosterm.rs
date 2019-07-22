use super::FooterOption;
use crate::search;
use crate::search::pageprinter::printer::{todoprinter::TodoPrinter, ItemPrinter};
use crate::search::term::SearchTerm;
use console::{style, Term};
use std::io::Result as IOResult;
use types::Todo;

#[derive(Clone)]
pub struct AllTodosTerm {
    term: Term,
    items: Vec<Todo>,
    printer: TodoPrinter,
}

impl SearchTerm<Todo, TodoPrinter> for AllTodosTerm {
    fn new(items: Vec<Todo>, term: Term) -> AllTodosTerm {
        AllTodosTerm {
            term: term,
            items: items,
            printer: TodoPrinter::new(),
        }
    }

    fn get_items(&self) -> &Vec<Todo> {
        &self.items
    }

    fn get_printer(&self) -> &TodoPrinter {
        &self.printer
    }

    fn set_on_quit(self, _: fn(_: Term) -> IOResult<()>) -> AllTodosTerm {
        self
    }

    fn char_match(&self, c: char) -> IOResult<bool> {
        match c {
            _ => Ok(false),
        }
    }

    fn on_quit(&self) -> IOResult<()> {
        search::start()
    }

    fn get_footer_options(&self) -> Vec<FooterOption> {
        vec![]
    }

    fn headline(&self) -> String {
        format!("{}", style("All ToDos").bold())
    }
}