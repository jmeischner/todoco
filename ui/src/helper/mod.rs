use console::{style, Term};
use log::debug;
use std::cmp;
use std::io::Result;
use types::Todo;
pub fn hbar(term: &Term) -> Result<()> {
    let width = get_term_width(term);

    let hbar = format!("{}", style("-".repeat(width as usize)).blue());
    term.write_line(&hbar)
}

pub fn print_todos(term: &Term, todos: &[Todo]) -> Result<()> {
    for todo in todos {
        print_todo(term, todo)?;
    }

    Ok(())
}

pub fn get_term_width(term: &Term) -> usize {
    match term.size_checked() {
        Some((_, width)) => width as usize,
        None => 40,
    }
}

pub fn get_term_height(term: &Term) -> usize {
    match term.size_checked() {
        Some((height, _)) => height as usize,
        None => 10,
    }
}

fn print_todo(term: &Term, todo: &Todo) -> Result<()> {
    let list_marker = format!("{}", style("◗").green());

    term.write_line(&format!("{}{} {}", tab(2), list_marker, todo.text))
}

fn tab(times: usize) -> String {
    "  ".repeat(times)
}
