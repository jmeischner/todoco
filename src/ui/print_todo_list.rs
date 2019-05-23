use ansi_term::Color::{Blue, Cyan, Green, Yellow};

use todoco::{List, Project, Todo};

pub fn print_project(project: &Project) {
    hbar();
    let project_title = format!("{}", Blue.bold().underline().paint(&project.name));
    println!("{:^80}", project_title);

    for list in &project.lists {
        print_list(&list);
    }

    hbar();

    print_summary(&project);

    hbar();
}

fn print_list(list: &List) {
    println!(
        "{}{} {}",
        tab(1),
        Cyan.bold().paint("◍"),
        Cyan.underline().paint(&list.name)
    );

    for todo in &list.todos {
        print_todo(&todo);
    }
}

fn print_todo(todo: &Todo) {
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

fn print_summary(project: &Project) {
    let list_count = project.lists.len();
    let todo_count = project.lists.iter().fold(0, |acc, l| acc + l.todos.len());
    let summary_text = format!("Found {} ToDos in {} List(s)", todo_count, list_count);
    let summary = format!("{:>80}", summary_text);

    println!("{}", Green.paint(summary));
}

fn tab(times: usize) -> String {
    "  ".repeat(times)
}

fn hbar() {
    println!("{}", Blue.paint("-".repeat(80)));
}
