use console::Term;
use pager::Pager;

use printer::itemprinter::ItemPrinter;
use std::io::Result as IOResult;
mod pager;
pub mod printer;

#[derive(Clone)]
pub struct PagePrinter<'a, I: Clone, P: Clone> {
    pager: Pager<'a, I>,
    printer: &'a P,
    lines_on_screen: usize,
    term: Term,
}

// Todo: What if there is only one page to show?
impl<'a, I: Clone, P: Clone> PagePrinter<'a, I, P>
where
    P: ItemPrinter<I>,
{
    /// Constructur
    pub fn new(items: &'a Vec<I>, height: usize, printer: &'a P) -> PagePrinter<'a, I, P> {

        PagePrinter {
            pager: Pager::new(items, height),
            lines_on_screen: 0,
            term: Term::stdout(),
            printer: printer,
        }
    }

    /// Print items of current page
    pub fn print_current(&mut self) -> IOResult<()> {
        self.term.clear_last_lines(self.lines_on_screen)?;
        let items = self.pager.current();

        self.lines_on_screen = items.len();
        self.printer.print_items(&self.term, items)?;

        Ok(())
    }

    /// Print items of next page
    pub fn print_next(&mut self) -> IOResult<()> {
        self.term.clear_last_lines(self.lines_on_screen)?;
        let items = self.pager.next();

        self.lines_on_screen = items.len();
        self.printer.print_items(&self.term, items)?;

        Ok(())
    }

    /// Print items of previous page
    pub fn print_prev(&mut self) -> IOResult<()> {
        self.term.clear_last_lines(self.lines_on_screen)?;
        let items = self.pager.prev();

        self.lines_on_screen = items.len();
        self.printer.print_items(&self.term, items)?;

        Ok(())
    }
}