use super::ItemPrinter;
use std::io::Result as IOResult;
use console::Term;
use std::marker::PhantomData;

#[derive(Clone)]
pub struct TextPrinter {
    name: PhantomData<String>
}

impl ItemPrinter<String> for TextPrinter {
    fn new() -> TextPrinter {
        TextPrinter {
            name: PhantomData
        }
    }

    fn print_items(&self, term: &Term, items: &[String]) -> IOResult<()> {
        for line in items {
            term.write_line(&line)?;
        }

        Ok(())
    }

    fn get_item_height(&self) -> usize {
        1
    }
}