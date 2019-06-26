use log::warn;
use std::io::Result as IOResult;
use std::path::PathBuf;
use types::{project::updater, Config, FilterMatch, Project};



pub mod init;

pub mod list;
pub mod scan;

// Todo: add error propagation

/// Integration method for *todoco scan* option
///
/// # Arguments
/// * `path` - A *PathBuf* which holds the base path from where the files get scanned for ToDo comments
pub fn scan(path: PathBuf) -> Result<Project, &'static str> {
    if let Some(root_dir) = path.to_str() {
        let (is_project, config) = get_config_and_project_info_from(&path);

        let todos = scan::get_todos(root_dir, &config);

        let project = match todos {
            Ok(tds) => scan::build_project(tds, config),
            Err(_) => return Err("It was not possible to scan the files of the current path."),
        };

        if is_project {
            let mut saved_project = match Project::from_dir(&path) {
                Some(project) => project,
                None => {
                    warn!("It was not possible to read saved project informations.");
                    Project::new(String::new(), vec![])
                }
            };
            let project = updater::update_project(&mut saved_project, &project);
            if let Err(_) = export::project_to_path(&project, path.clone()) {
                return Err("It was not possible to export project results.");
            };
        };

        Ok(project)
    } else {
        Err("It was not possible to handle given path.")
    }
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
    let project = list::get_project(is_project, &current_dir)?;

    if let Some(keyword) = keyword {
        Ok(list::get_matching_todos(keyword, &project))
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
        Err(_) => (false, init::get_default_config(&path)),
    }
}