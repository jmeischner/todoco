use crate::helper;
use crate::search::pageprinter::printer::ItemPrinter;
use crate::search::pageprinter::{Page, PagePrinter};
use console::{style, Key, Term};
use std::io::Result as IOResult;
use std::marker::PhantomData;


pub mod alltodosterm;
pub mod mainterm;

/// Trait for implementing specific term ui
/// of the search cli verb
///
/// # Function
/// `new` - create a new instance of term
/// `get_items` - getter of items hold by the term
/// `get_printer` - getter of the printer corresponding to the term
/// `char_match` - matcher of keyboard input keys to corresponding actions
/// `on_quit` - action to perform when closing ui
/// `get_footer_options` - Options corresponding to `char_match` actions
/// `headline` - function returning the headline
pub trait SearchTerm<I: Clone, P: Clone>
where
    P: ItemPrinter<I>,
{
    fn new(items: Vec<I>, term: Term) -> Self;
    fn get_items(&self) -> &Vec<I>;
    fn get_printer(&self) -> &P;
    fn char_match(&self, c: char) -> IOResult<bool>;
    fn on_quit(&self) -> IOResult<()>;
    fn get_footer_options(&self) -> Vec<FooterOption>;
    fn headline(&self) -> String;
}

/// Struct for giving the `TermDialog`
/// an overview of the `SearchTerm`
/// footer options
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

/// Struct which is the outer shell
/// of the interactive `SearchTerm`
/// solution
///
/// # Properties
/// `search` - Holds an instance of an SearchTerm
/// `term` - Instance of console::Term
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
            term: term,
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
        self.header()?;
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
                    self.header()?;
                    page_printer.print(Page::Next)?;
                    self.footer()?;
                }
                Key::ArrowUp | Key::ArrowLeft => {
                    self.clear()?;
                    self.header()?;
                    page_printer.print(Page::Previous)?;
                    self.footer()?;
                }
                _ => {}
            }
        }

        self.search.on_quit()
    }

    fn get_term_height(&self) -> usize {
        helper::get_term_height(&self.term)
            - self.get_number_of_header_lines()
            - self.get_number_of_footer_lines()
            - 1 // Line for read_key
    }

    fn get_page_printer(&self) -> PagePrinter<I, P> {
        let items = self.search.get_items();
        let height = self.get_term_height();
        let printer = self.search.get_printer();

        PagePrinter::new(items, height, printer)
    }

    fn footer(&self) -> IOResult<()> {
        let term_options = self.search.get_footer_options();

        let mut options = vec![
            FooterOption::new("▶|▼", "Next Page"),
            FooterOption::new("◀|▲", "Previous Page"),
        ];

        options.extend(term_options);

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

    fn header(&self) -> IOResult<()> {
        let line = self.search.headline();

        helper::hbar(&self.term)?;
        self.term.write_line(&line)?;
        helper::hbar(&self.term)
    }

    // Todo: Handle if term is not as wide as it should be to display full footer line
    fn get_number_of_header_lines(&self) -> usize {
        3
    }

    fn get_number_of_footer_lines(&self) -> usize {
        2
    }
}