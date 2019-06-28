use ansi_term::Color::{Blue, Green};

use types::{Project};

use super::helper_old::*;

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

fn print_summary(project: &Project) {
    let todo_count = project.todos.len();
    let summary_text = format!("Found {} ToDos", todo_count);
    let summary = format!("{:>80}", summary_text);

    println!("{}", Green.paint(summary));
}
