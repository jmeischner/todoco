
use crate::helper;
use console::{style, Key, Term};

use pageprinter::printer::itemprinter::ItemPrinter;
use pageprinter::printer::todoprinter::TodoPrinter;
use pageprinter::PagePrinter;
use std::io::Result as IOResult;
use types::Project;
pub mod pageprinter;
mod term;

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
        let area_height = helper::get_term_height(&self.term) - HEADER_LINES;
        let mut page_printer =
            PagePrinter::new(&self.project.todos, area_height, TodoPrinter::new());
        page_printer.print_current()?;
        self.print_list_all_menu(true)?;

        loop {
            let key = self.term.read_key()?;
            match key {
                Key::Char(c) => {
                    if c == 'q' {
                        break;
                    }
                }
                Key::ArrowDown | Key::ArrowRight => {
                    page_printer.print_next()?;
                    self.print_list_all_menu(false)?;
                }
                Key::ArrowUp | Key::ArrowLeft => {
                    page_printer.print_prev()?;
                    self.print_list_all_menu(false)?;
                }
                _ => {}
            }
        }


        self.start()
    }

    fn print_list_all_menu(&self, initial: bool) -> IOResult<()> {
        if !initial {
            self.term.clear_last_lines(2)?;
        };

        helper::hbar(&self.term)?;
        let menu_line = format!(
            "{}: Next Page | {}: Previous Page | {}: Quit",
            style("▶|▼").blue(),
            style("◀|▲").blue(),
            style("q").blue(),
        );
        self.term.write_line(&menu_line)
    }

}