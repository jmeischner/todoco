use crate::terminal::ItemPrinter;

use console::Term;
use std::io::Result as IOResult;
use types::Tag;

#[derive(Clone)]
pub struct TagPrinter {}

impl ItemPrinter<Tag> for TagPrinter {
    fn new() -> TagPrinter {
        TagPrinter {}
    }

    fn print_items(&self, term: &Term, items: &[Tag]) -> IOResult<()> {
        for tag in items {
            term.write_line(&format!("{}", tag.name))?;
        }

        Ok(())
    }

    fn get_item_height(&self) -> usize {
        1
    }
}