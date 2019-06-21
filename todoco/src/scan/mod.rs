use types::{Todo, Config, Project};
use std::io::Result as IOResult;

mod handle_files;

pub fn get_todos(dir: &str, config: &Config) -> IOResult<Vec<Todo>> {
    let files = handle_files::get_files(dir, config);
    handle_files::extract_todos_from_files(files)
}

pub fn build_project(todos: Vec<Todo>, config: Config) -> Project {
    Project::new(config.project.name, todos)
}
