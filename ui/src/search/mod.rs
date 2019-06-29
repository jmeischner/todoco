use crate::search::term::mainterm::MainTerm;
use crate::search::term::TermDialog;
use console::Term;
use pageprinter::printer::itemprinter::ItemPrinter;

use pageprinter::printer::textprinter::TextPrinter;
use std::io::Result as IOResult;
use term::SearchTerm;
pub mod pageprinter;
pub mod term;

pub fn start() -> IOResult<()> {
    let welcome_text: Vec<String> = include_str!("../../welcome_search.txt")
            .split("\n")
            .map(|line| line.to_string())
            .collect();
    let printer = TextPrinter::new();
    let main_term = MainTerm::new(welcome_text, printer);
    let term = Term::stdout();
    let dialog = TermDialog::new(term, main_term);
    dialog.start()
}