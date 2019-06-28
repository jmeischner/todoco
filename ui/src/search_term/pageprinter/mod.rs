use crate::helper;
use console::Term;
use pager::Pager;

use std::io::Result as IOResult;
use types::Todo;
mod pager;

pub struct PagePrinter<'a> {
    pager: Pager<'a>,
    lines_on_screen: usize,
    term: Term,
}

impl<'a> PagePrinter<'a> {
    /// Constructur
    pub fn new(items: &'a Vec<Todo>, height: usize) -> PagePrinter {
        PagePrinter {
            pager: Pager::new(items, height),
            lines_on_screen: 0,
            term: Term::stdout(),
        }
    }

    /// Print items of current page
    pub fn print_current(&mut self) -> IOResult<()> {
        self.term.clear_last_lines(self.lines_on_screen)?;
        let items = self.pager.current();

        self.lines_on_screen = items.len();
        helper::print_todos(&self.term, items)?;

        Ok(())
    }

    /// Print items of next page
    pub fn print_next(&mut self) -> IOResult<()> {
        self.term.clear_last_lines(self.lines_on_screen)?;
        let items = self.pager.next();

        self.lines_on_screen = items.len();
        helper::print_todos(&self.term, items)?;

        Ok(())
    }

    /// Print items of previous page
    pub fn print_prev(&mut self) -> IOResult<()> {
        self.term.clear_last_lines(self.lines_on_screen)?;
        let items = self.pager.prev();

        self.lines_on_screen = items.len();
        helper::print_todos(&self.term, items)?;

        Ok(())
    }
}