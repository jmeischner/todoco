use std::io::Result as IOResult;
use std::path::PathBuf;
use todofilter;
use types::{config, Config, FilterMatch, Project};

// Todo: add error propagation

/// Integration method for *todoco scan* option
///
/// # Arguments
/// * `path` - A *PathBuf* which holds the base path from where the files get scanned for ToDo comments
pub fn scan(path: PathBuf) -> Result<Project, &'static str> {
    todoscanner::scan(path)
}

/// Integration method for *todoco init* option
///
/// # Arguments
/// * `config` - The config given by the given answers
/// * `path` - A *PathBuf* where the new todoco project should get initialized
pub fn init(config: Config, path: PathBuf) -> IOResult<()> {
    config.write(&path)?;
    export::init_project_dir(path)?;
    Ok(())
}

/// Inttegration method for *todoco list* option
///
/// # Arguments
/// * `keyword` - The keyword the todos should get filtered for
pub fn list(keyword: Option<&str>, current_dir: PathBuf) -> Result<FilterMatch, &'static str> {

    let (is_project, _config) = get_config_and_project_info_from(&current_dir);
    let project = todofilter::get_project(is_project, &current_dir)?;

    if let Some(keyword) = keyword {
        Ok(todofilter::get_matching_todos(keyword, &project))
    } else {
        match project.todos.len() {
            0 => Ok(FilterMatch::None),
            _ => Ok(FilterMatch::All(project.todos)),
        }
    }
}

fn get_config_and_project_info_from(path: &PathBuf) -> (bool, Config) {
    match Config::from_dir(&path) {
        Ok(c) => (true, c),
        Err(_) => (false, config::get_default_config(&path)),
    }
}