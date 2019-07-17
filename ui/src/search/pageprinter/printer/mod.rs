pub mod tagprinter;
pub mod textprinter;
pub mod todoprinter;

use console::Term;
use std::io::Result as IOResult;

pub trait ItemPrinter<I: Clone> {
    fn new() -> Self;
    fn print_items(&self, term: &Term, items: &[I]) -> IOResult<()>;
}