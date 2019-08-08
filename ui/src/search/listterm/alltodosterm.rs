use console::{style, Term};
use crate::search;
use crate::search::TodoPrinter;
use crate::terminal::{FooterOption, ItemPrinter, SearchTerm};
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

    fn get_term(&self) -> &Term {
        &self.term
    }

    fn set_on_quit(&mut self, _: fn(_: Self, _: bool) -> IOResult<()>) {}

    fn char_match(&self, c: char) -> IOResult<bool> {
        match c {
            _ => Ok(false),
        }
    }

    fn on_quit(&self, _: bool) -> IOResult<()> {
        search::start()
    }

    fn get_footer_options(&self) -> Vec<FooterOption> {
        vec![]
    }

    fn headline(&self) -> String {
        format!("{}", style("All ToDos").bold())
    }
}