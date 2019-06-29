use std::io::Result as IOResult;
use console::Term;

pub trait ItemPrinter<I> {
    fn new() -> Self;
    fn print_items(&self, term: &Term, items: &[I]) -> IOResult<()>;
}