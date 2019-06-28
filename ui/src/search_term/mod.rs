use crate::helper;
use console::{Key, Term};
use std::io::Result as IOResult;
use types::Project;
use pageprinter::PagePrinter;

mod pageprinter;

const HEADER_LINES: usize = 4;

pub fn start(project: &Project) -> IOResult<()> {
    let term = Term::stdout();
    term.clear_screen()?;

    helper::hbar(&term)?;
    term.write_line("Search: ")?;
    helper::hbar(&term)?;

    result_area(&term, &project)?;

    Ok(())
}

fn result_area(term: &Term, project: &Project) -> IOResult<()> {
    let area_height = helper::get_term_height(term) - HEADER_LINES;
    let mut page_printer = PagePrinter::new(&project.todos, area_height);
    page_printer.print_current()?;

    loop {
        let key = term.read_key()?;
        match key {
            Key::Enter => break,
            Key::ArrowDown | Key::ArrowRight => page_printer.print_next()?,
            Key::ArrowUp | Key::ArrowLeft => page_printer.print_prev()?,
            _ => {}
        }
    }

    Ok(())
}