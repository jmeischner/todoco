use super::itemprinter::ItemPrinter;
use types::Tag;
use std::io::Result as IOResult;
use console::Term;

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
}