use ansi_term::Color::{Green, Red};

use types::{FilterMatch, Todo};

use super::helper_old::*;

pub fn print(matches: FilterMatch) {
    match matches {
        FilterMatch::None => print_none(),
        FilterMatch::Tags(todos) => print_by_tags(todos),
        FilterMatch::Files(todos) => print_by_files(todos),
        FilterMatch::Text(todos) => print_by_todo_text(todos),
        FilterMatch::All(todos) => print_all_todos(todos),
    }
}

fn print_none() {
    hbar();
    println!("{}", Red.paint("No ToDos Found!"));
    hbar();
}

fn print_by_tags(todos: Vec<Todo>) {
    hbar();
    let tags = todos
        .iter()
        .flat_map(|todo| &todo.tags)
        .fold(String::new(), |res, tag| format!("{} {}", res, tag.name));
    let summary_text = format!("Found Todos with following Tags:\n{}", tags);
    println!("{}\n", Green.paint(summary_text));
    for todo in todos {
        print_todo(&todo);
    }
    hbar();
}

// Todo: use group_by_file
fn print_by_files(todos: Vec<Todo>) {
    hbar();
    let summary_text = format!("Found Todos by matching file names");
    println!("{}\n", Green.paint(summary_text));
    for todo in todos {
        print_todo(&todo);
    }
    hbar();
}

fn print_by_todo_text(todos: Vec<Todo>) {
    hbar();
    let summary_text = format!("Found Todos by matching text in todo");
    println!("{}\n", Green.paint(summary_text));
    for todo in todos {
        print_todo(&todo);
    }
    hbar();
}

fn print_all_todos(todos: Vec<Todo>) {
    hbar();
    let summary_text = format!("All ToDos");
    println!("{}\n", Green.paint(summary_text));
    for todo in todos {
        print_todo(&todo);
    }
    hbar();
}