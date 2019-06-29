use crate::helper;
use crate::search::pageprinter::printer::itemprinter::ItemPrinter;
use crate::search::pageprinter::PagePrinter;
use console::{Key, Term};
use std::io::Result as IOResult;
use std::marker::PhantomData;

pub mod mainterm;
pub mod alltodosterm;

pub trait SearchTerm<I: Clone, P: Clone>
where
    P: ItemPrinter<I>,
{
    fn new(items: Vec<I>, printer: P) -> Self;
    fn set_term(self, term: Term) -> Self;
    fn get_term(&self) -> Term;
    fn get_items(&self) -> &Vec<I>;
    fn get_printer(&self) -> &P;
    fn header(&self) -> IOResult<()>;
    fn char_match(&self, c: char) -> IOResult<bool>;
    fn main(&self) -> IOResult<()> {
        self.header()?;
        let items = &self.get_items();
        let height = helper::get_term_height(&self.get_term()) - self.get_header_lines();
        let printer = self.get_printer();
        let mut page_printer = PagePrinter::new(items, height, printer).clone();
        page_printer.print_current()?;
        self.term_footer(true)?;

        loop {
            let key = self.get_term().read_key()?;
            match key {
                Key::Char(c) => {
                    if let Ok(should_break) = self.char_match(c) {
                        if should_break {
                            break;
                        }
                    }
                }
                Key::ArrowDown | Key::ArrowRight => {
                    page_printer.print_next()?;
                    self.term_footer(false)?;
                }
                Key::ArrowUp | Key::ArrowLeft => {
                    page_printer.print_prev()?;
                    self.term_footer(false)?;
                }
                _ => {}
            }
        }

        self.on_loop_end()
    }
    fn on_loop_end(&self) -> IOResult<()>;
    fn term_footer(&self, initial: bool) -> IOResult<()> {
        if !initial {
            self.get_term().clear_last_lines(self.get_footer_lines())?;
        }

        self.footer()
    }
    fn footer(&self) -> IOResult<()>;
    fn get_header_lines(&self) -> usize;
    fn get_footer_lines(&self) -> usize;
}

pub struct TermDialog<I, P, S> {
    items: PhantomData<I>,
    printer: PhantomData<P>,
    search: S,
    term: Term,
}

impl<I: Clone, P: ItemPrinter<I> + Clone, S: SearchTerm<I, P> + Clone> TermDialog<I, P, S> {
    pub fn new(term: Term, search: S) -> TermDialog<I, P, S> {
        TermDialog {
            items: PhantomData,
            printer: PhantomData,
            term: term.clone(),
            search: search.set_term(term),
        }
    }

    pub fn start(&self) -> IOResult<()> {
        self.clear()?;
        self.search.main()
    }

    fn clear(&self) -> IOResult<()> {
        self.term.clear_screen()
    }
}