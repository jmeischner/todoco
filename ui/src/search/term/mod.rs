use console::{Key,Term};
use std::io::Result as IOResult;
use crate::helper;
use crate::search::pageprinter::PagePrinter;
use crate::search::pageprinter::printer::itemprinter::ItemPrinter;

pub trait SearchTerm {
    fn new() -> Self;
    fn set_term(self, term: &Term) -> Self;
    fn header(&self) -> IOResult<()>;
    fn char_match(&self, c: char) -> IOResult<bool>;
    fn on_loop_end(&self) -> IOResult<()>;
    fn footer(&self) -> IOResult<()>;
    fn get_header_lines(&self) -> usize;
    fn get_footer_lines(&self) -> usize;
}

pub struct TermDialog<S, I, P>
{
    search: S,
    term: &'static Term,
    content: Vec<I>,
    printer: P
}

impl<S,I,P> TermDialog<S,I,P>
where 
    S: SearchTerm,
    P: ItemPrinter<I>
{
    pub fn new(term: &'static Term, content: Vec<I>, search: S, printer: P) -> TermDialog<S,I,P> {
        TermDialog {
            term: term,
            content: content,
            search: search.set_term(&term),
            printer: printer
        }
    }

    pub fn start(&self) -> IOResult<()> {
        self.clear()?;
        self.main()
    }

    fn clear(&self) -> IOResult<()> {
        self.term.clear_screen()
    }

    fn main(&self) -> IOResult<()> {
        self.search.header()?;
        let height = helper::get_term_height(&self.term) - self.search.get_header_lines();
        let mut page_printer = PagePrinter::new(&self.content, height, &self.printer);
        page_printer.print_current()?;
        self.footer(true)?;

        loop {
            let key = self.term.read_key()?;
            match key {
                Key::Char(c) => {
                    if let Ok(should_break) = self.search.char_match(c) {
                        if should_break {
                            break;
                        }
                    }
                }
                Key::ArrowDown | Key::ArrowRight => {
                    page_printer.print_next()?;
                    self.footer(false)?;
                }
                Key::ArrowUp | Key::ArrowLeft => {
                    page_printer.print_prev()?;
                    self.footer(false)?;
                }
                _ => {}
            }
        }

        self.search.on_loop_end()
    }

    fn footer(&self, initial: bool) -> IOResult<()> {
        if !initial {
            self.term.clear_last_lines(self.search.get_footer_lines())?;
        }

        self.search.footer()
    }
}