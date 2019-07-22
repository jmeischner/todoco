use console::{Emoji, style, Term};
use std::io::Result;

pub fn hbar(term: &Term) -> Result<()> {
    let width = get_term_width(term);

    let hbar = format!("{}", style("-".repeat(width as usize)).blue());
    term.write_line(&hbar)
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

pub fn tab(times: usize) -> String {
    "  ".repeat(times)
}

pub fn get_goodbye_message() -> String {
    format!(
        "Goodbye {} and Thank You {}",
        Emoji("👋", ""),
        Emoji("🙏", "")
    )
}