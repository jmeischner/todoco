use super::FooterOption;
use crate::search;
use crate::search::pageprinter::printer::{tagprinter::TagPrinter, ItemPrinter};
use crate::search::term::SearchTerm;
use console::{style, Term};
use std::io::Result as IOResult;
use types::Tag;

#[derive(Clone)]
pub struct AllTagsTerm {
    term: Term,
    items: Vec<Tag>,
    printer: TagPrinter,
}

impl SearchTerm<Tag, TagPrinter> for AllTagsTerm {
    fn new(items: Vec<Tag>, term: Term) -> AllTagsTerm {
        AllTagsTerm {
            term: term,
            items: items,
            printer: TagPrinter::new(),
        }
    }

    fn get_items(&self) -> &Vec<Tag> {
        &self.items
    }

    fn get_printer(&self) -> &TagPrinter {
        &self.printer
    }

    fn set_on_quit(self, _: fn() -> IOResult<()>) -> AllTagsTerm {
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
        format!("{}", style("All Tags").bold())
    }
}