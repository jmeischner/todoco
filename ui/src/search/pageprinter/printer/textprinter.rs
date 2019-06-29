use super::itemprinter::ItemPrinter;
use std::io::Result as IOResult;
use console::Term;

pub struct TextPrinter {}

impl ItemPrinter<String> for TextPrinter {
    fn new() -> TextPrinter {
        TextPrinter {}
    }

    fn print_items(&self, term: &Term, items: &[String]) -> IOResult<()> {
        for line in items {
            term.write_line(&line)?;
        }

        Ok(())
    }
}