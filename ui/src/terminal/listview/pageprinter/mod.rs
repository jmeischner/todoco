use console::Term;
pub use itemprinter::ItemPrinter;
use pager::Pager;
use std::io::Result as IOResult;

mod itemprinter;
mod pager;
/// Enum to Identify which Page
/// the PagePrinter should show
///
/// # Possible Values
/// - Current
/// - Next
/// - Previous
///
pub enum Page {
    Current,
    Next,
    Previous,
}

/// Struct which handles the
/// output of pages of items
///
/// # Properties
/// `pager` - The Struct which returns the
/// items of one page
/// `printer` - The Printer which handles the
/// logic of printing a list of the given item types
/// `term` - Holds the output term
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
        let item_height = printer.get_item_height();
        let pager = Pager::new(items, height, item_height);

        PagePrinter {
            pager: pager,
            term: Term::stdout(),
            printer: printer,
        }
    }

    pub fn print(&mut self, page: Page) -> IOResult<()> {
        let items = match page {
            Page::Current => self.pager.current(),
            Page::Next => self.pager.next(),
            Page::Previous => self.pager.prev(),
        };

        let count = items.len();

        self.printer.print_items(&self.term, items)?;
        self.print_missing_empty_lines(count)?;
        Ok(())
    }

    fn print_missing_empty_lines(&self, cur_lines: usize) -> IOResult<()> {
        let missing_lines = &self.pager.page_height() - cur_lines * self.printer.get_item_height();
        for _ in 0..missing_lines {
            self.term.write_line("")?;
        }

        Ok(())
    }
}