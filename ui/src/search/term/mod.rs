use crate::helper;
use crate::search::pageprinter::printer::itemprinter::ItemPrinter;
use crate::search::pageprinter::{Page, PagePrinter};
use console::{style, Key, Term};
use std::io::Result as IOResult;
use std::marker::PhantomData;


pub mod alltodosterm;
pub mod mainterm;

pub trait SearchTerm<I: Clone, P: Clone>
where
    P: ItemPrinter<I>,
{
    fn new(items: Vec<I>, printer: P, term: Term) -> Self;
    fn get_items(&self) -> &Vec<I>;
    fn get_printer(&self) -> &P;
    fn header(&self) -> IOResult<()>;
    fn char_match(&self, c: char) -> IOResult<bool>;
    fn on_loop_end(&self) -> IOResult<()>;
    fn get_footer_options(&self) -> Vec<FooterOption>;
    fn get_header_lines(&self) -> usize;
    // Todo: This should come from TermDialog
    fn get_footer_lines(&self) -> usize;
}

pub struct FooterOption {
    key: String,
    description: String,
}

impl FooterOption {
    pub fn new(key: &str, description: &str) -> FooterOption {
        FooterOption {
            key: key.to_string(),
            description: description.to_string(),
        }
    }
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
            search: search,
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
        let mut page_printer = self.get_page_printer();

        // Print Start Page
        self.search.header()?;
        page_printer.print(Page::Current)?;
        self.footer()?;

        // Start interaction loop
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
                    self.clear()?;
                    self.search.header()?;
                    page_printer.print(Page::Next)?;
                    self.footer()?;
                }
                Key::ArrowUp | Key::ArrowLeft => {
                    self.clear()?;
                    self.search.header()?;
                    page_printer.print(Page::Previous)?;
                    self.footer()?;
                }
                _ => {}
            }
        }

        self.search.on_loop_end()
    }

    fn get_term_height(&self) -> usize {
        // Todo: Handle if term is not as wide as it should be to display full footer line
        helper::get_term_height(&self.term)
            - self.search.get_header_lines()
            - self.search.get_footer_lines()
            - 1 // Line for read_key
    }

    fn get_page_printer(&self) -> PagePrinter<I, P> {
        let items = self.search.get_items();
        let height = self.get_term_height();
        let printer = self.search.get_printer();
        // Todo: Refactor .clone()
        PagePrinter::new(items, height, printer)
    }

    fn footer(&self) -> IOResult<()> {
        let term_options = self.search.get_footer_options();

        let mut options = vec![
            FooterOption::new("▶|▼", "Next Page"),
            FooterOption::new("◀|▲", "Previous Page"),
        ];

        options.extend(term_options);

        options.extend(vec![FooterOption::new("q", "Quit")]);

        let mut footer = options.iter().rev().fold(String::new(), |text, option| {
            format!(
                "{}: {} | {}",
                style(&option.key).blue(),
                option.description,
                text
            )
        });

        // Todo: find better method for cut of last "| "
        let len = footer.len();
        footer.truncate(len - 2);

        helper::hbar(&self.term)?;
        self.term.write_line(&footer)
    }
}