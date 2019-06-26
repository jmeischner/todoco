use types::{Todo, Config, Project};
use std::io::Result as IOResult;
use std::path::Path;
use log::warn;

mod handle_files;

pub fn get_todos(dir: &str, config: &Config) -> IOResult<Vec<Todo>> {
    let files = handle_files::get_files(dir, config);
    handle_files::extract_todos_from_files(files)
}

pub fn build_project(todos: Vec<Todo>, config: Config) -> Project {
    Project::new(config.project.name, todos)
}

pub fn get_saved_project(path: &Path) -> Project {
    match Project::from_dir(path) {
        Some(project) => project,
        None => {
            warn!("It was not possible to read saved project informations.");
            Project::new(String::new(), vec![])
        }
    }
}