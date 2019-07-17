use crate::helper;
use crate::search;
use crate::search::pageprinter::printer::todoprinter::TodoPrinter;
use crate::search::term::SearchTerm;
use console::{Term};
use std::io::Result as IOResult;
use super::FooterOption;
use types::Todo;

#[derive(Clone)]
pub struct AllTodosTerm {
    term: Term,
    items: Vec<Todo>,
    printer: TodoPrinter,
}

impl SearchTerm<Todo, TodoPrinter> for AllTodosTerm {
    fn new(items: Vec<Todo>, printer: TodoPrinter, term: Term) -> AllTodosTerm {
        AllTodosTerm {
            term: term,
            items: items,
            printer: printer,
        }
    }

    fn get_items(&self) -> &Vec<Todo> {
        &self.items
    }

    fn get_printer(&self) -> &TodoPrinter {
        &self.printer
    }

    fn header(&self) -> IOResult<()> {
        helper::hbar(&self.term)
    }


    fn char_match(&self, c: char) -> IOResult<bool> {
        match c {
            'q' => Ok(true),
            _ => Ok(false),
        }
    }

    fn on_loop_end(&self) -> IOResult<()> {
        search::start()
    }

    fn get_footer_options(&self) -> Vec<FooterOption> {
        vec![]
    }

    fn get_header_lines(&self) -> usize {
        1
    }

    fn get_footer_lines(&self) -> usize {
        2
    }
}