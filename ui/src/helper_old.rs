use ansi_term::Color::{Blue, Yellow, Green};
use types::Todo;

pub fn tab(times: usize) -> String {
    "  ".repeat(times)
}

pub fn hbar() {
    println!("{}", Blue.paint("-".repeat(80)));
}

pub fn print_todo(todo: &Todo) {
    // Todo: break longer texts
    let todo_text = format!("{}", todo.text);

    println!("{}{} {}", tab(2), Green.paint("◗"), todo_text);

    // Todo: make configurable
    let path_text = format!("at {}:{}", todo.file.path, todo.line);
    println!("{}{}", tab(3), Yellow.paint(path_text));

    for tag in &todo.tags {
        if let Some(value) = &tag.value {
            println!("{}Tag: {}, Value: {}", tab(3), &tag.name, value);
        } else {
            println!("{}Tag: {}", tab(3), &tag.name);
        }
    }
}