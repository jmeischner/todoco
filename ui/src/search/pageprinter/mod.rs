use console::Term;
use log::debug;
use pager::Pager;
use printer::itemprinter::ItemPrinter;
use std::io::Result as IOResult;


mod pager;
pub mod printer;

#[derive(Clone)]
pub struct PagePrinter<'a, I: Clone, P: Clone> {
    pager: Pager<'a, I>,
    printer: &'a P,
    term: Term,
}

// Todo: What if there is only one page to show?
impl<'a, I: Clone, P: Clone> PagePrinter<'a, I, P>
where
    P: ItemPrinter<I>,
{
    /// Constructur
    pub fn new(items: &'a Vec<I>, height: usize, printer: &'a P) -> PagePrinter<'a, I, P> {

        let pager = Pager::new(items, height);

        PagePrinter {
            pager: pager,
            term: Term::stdout(),
            printer: printer,
        }
    }

    /// Print items of current page
    pub fn print_start_page(&mut self) -> IOResult<()> {
        let items = self.pager.current();
        let items_count = items.len();

        self.printer.print_items(&self.term, items)?;
        self.print_missing_empty_lines(items_count)?;
        Ok(())
    }

    /// Print items of next page
    pub fn print_next(&mut self) -> IOResult<()> {
        self.term.clear_last_lines(self.pager.page_height())?;
        let items = self.pager.next();
        let items_count = items.len();

        self.printer.print_items(&self.term, items)?;
        self.print_missing_empty_lines(items_count)?;
        Ok(())
    }

    /// Print items of previous page
    pub fn print_prev(&mut self) -> IOResult<()> {
        self.term.clear_last_lines(self.pager.page_height())?;
        let items = &self.pager.prev();
        let items_count = items.len();

        self.printer.print_items(&self.term, items)?;
        self.print_missing_empty_lines(items_count)?;
        Ok(())
    }

    fn print_missing_empty_lines(&self, cur_lines: usize) -> IOResult<()> {
        // debug!(
        //     "height: {}, cur_lines: {}",
        //     &self.pager.page_height(),
        //     cur_lines
        // );
        let missing_lines = &self.pager.page_height() - cur_lines;
        for _ in 0..missing_lines {
            self.term.write_line("")?;
        }

        Ok(())
    }
}