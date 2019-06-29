use crate::helper;
use crate::search;
use crate::search::pageprinter::printer::todoprinter::TodoPrinter;
use crate::search::term::SearchTerm;
use console::{style, Term};
use std::io::Result as IOResult;
use types::Todo;

#[derive(Clone)]
pub struct AllTodosTerm {
    term: Option<Term>,
    items: Vec<Todo>,
    printer: TodoPrinter,
}

impl SearchTerm<Todo, TodoPrinter> for AllTodosTerm {
    fn new(items: Vec<Todo>, printer: TodoPrinter) -> AllTodosTerm {
        AllTodosTerm {
            term: None,
            items: items,
            printer: printer,
        }
    }

    fn set_term(mut self, term: Term) -> AllTodosTerm {
        self.term = Some(term);
        self
    }

    fn get_term(&self) -> Term {
        if let Some(term) = &self.term {
            term.clone()
        } else {
            Term::stdout()
        }
    }

    fn get_items(&self) -> &Vec<Todo> {
        &self.items
    }

    fn get_printer(&self) -> &TodoPrinter {
        &self.printer
    }

    fn header(&self) -> IOResult<()> {
        helper::hbar(&self.get_term())
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

    fn footer(&self) -> IOResult<()> {
        helper::hbar(&self.get_term())?;
        let menu_line = format!(
            "{}: Next Page | {}: Previous Page | {}: Quit",
            style("▶|▼").blue(),
            style("◀|▲").blue(),
            style("q").blue(),
        );
        self.get_term().write_line(&menu_line)
    }

    fn get_header_lines(&self) -> usize {
        4
    }

    fn get_footer_lines(&self) -> usize {
        2
    }
}