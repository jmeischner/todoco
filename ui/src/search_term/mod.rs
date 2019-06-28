
use crate::helper;
use console::{style, Key, Term};
use pageprinter::printer::itemprinter::ItemPrinter;
use pageprinter::printer::todoprinter::TodoPrinter;
use pageprinter::PagePrinter;

use std::io::Result as IOResult;
use types::Project;
mod pageprinter;

const HEADER_LINES: usize = 4;

pub struct SearchTerm {
    term: Term,
    project: Project,
}

impl SearchTerm {
    pub fn new(project: Project) -> SearchTerm {
        SearchTerm {
            term: Term::stdout(),
            project: project,
        }
    }

    pub fn start(&self) -> IOResult<()> {
        self.clear()?;
        self.welcome()?;
        self.print_main_menu()?;
        self.start_main_event_loop()
    }

    fn clear(&self) -> IOResult<()> {
        self.term.clear_screen()
    }

    fn welcome(&self) -> IOResult<()> {
        let welcome_line = format!("{}", style("What do you want to do?").bold());
        self.term.write_line(&welcome_line)
    }

    fn print_main_menu(&self) -> IOResult<()> {
        let menu_line = format!(
            "{}: List All | {}: Search by Keyword | {}: List Tags | {}: Quit",
            style("a").blue(),
            style("k").blue(),
            style("t").blue(),
            style("q").blue(),
        );
        self.term.write_line(&menu_line)
    }

    fn start_main_event_loop(&self) -> IOResult<()> {
        loop {
            let key = self.term.read_key()?;
            match key {
                Key::Char(c) => match c {
                    'q' => break,
                    'a' => {
                        self.list_all()?;
                        break;
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        Ok(())
    }

    fn list_all(&self) -> IOResult<()> {
        self.clear()?;
        helper::hbar(&self.term)?;
        let area_height = helper::get_term_height(&self.term) - 2;
        let printer = TodoPrinter::new();
        let mut page_printer = PagePrinter::new(&self.project.todos, area_height, printer);
        page_printer.print_current()?;

        loop {
            let key = self.term.read_key()?;
            match key {
                Key::Char(c) => {
                    if c == 'q' {
                        break;
                    }
                }
                Key::ArrowDown | Key::ArrowRight => page_printer.print_next()?,
                Key::ArrowUp | Key::ArrowLeft => page_printer.print_prev()?,
                _ => {}
            }
        }

        self.start()
    }

}