use console::Term;
use std::io::Result as IOResult;
use super::pageprinter::ItemPrinter;
use super::super::FooterOption;

/// Trait for implementing specific term ui
/// of the search cli verb
///
/// # Function
/// `new` - create a new instance of term
/// `get_items` - getter of items hold by the term
/// `get_printer` - getter of the printer corresponding to the term
/// `char_match` - matcher of keyboard input keys to corresponding actions
/// `on_quit` - action to perform when closing ui
/// `get_footer_options` - Options corresponding to `char_match` actions
/// `headline` - function returning the headline
pub trait SearchTerm<I: Clone, P: Clone>
where
    P: ItemPrinter<I>,
{
    fn new(items: Vec<I>, term: Term) -> Self;
    fn get_items(&self) -> &Vec<I>;
    fn get_printer(&self) -> &P;
    fn get_term(&self) -> &Term;
    fn char_match(&self, c: char) -> IOResult<bool>;
    fn set_on_quit(&mut self, f: fn(current: Self, by_escape: bool) -> IOResult<()>);
    fn on_quit(&self, by_escape: bool) -> IOResult<()>;
    fn get_footer_options(&self) -> Vec<FooterOption>;
    fn headline(&self) -> String;
}