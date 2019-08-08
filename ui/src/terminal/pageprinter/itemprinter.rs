use console::Term;
use std::io::Result as IOResult;

pub trait ItemPrinter<I: Clone> {
    // Todo: Is it neccessary to have a contructor?
    fn new() -> Self;
    fn print_items(&self, term: &Term, items: &[I]) -> IOResult<()>;
    fn get_item_height(&self) -> usize;
}