use ansi_term::Color::{Blue, Green, Yellow};

use types::{Project, Todo};

pub fn print_project(project: &Project) {
    hbar();
    let project_title = format!("{}", Blue.bold().underline().paint(&project.name));
    println!("{:^80}", project_title);

    for todo in &project.todos {
        print_todo(&todo);
    }

    hbar();

    print_summary(&project);

    hbar();
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
    let todo_count = project.todos.len();
    let summary_text = format!("Found {} ToDos", todo_count);
    let summary = format!("{:>80}", summary_text);

    println!("{}", Green.paint(summary));
}

fn tab(times: usize) -> String {
    "  ".repeat(times)
}

fn hbar() {
    println!("{}", Blue.paint("-".repeat(80)));
}
