use std::io::Result as IOResult;
use std::path::PathBuf;
use types::{Project, Config, project::updater};

pub mod init;
pub mod scan;

// Todo: add error propagation
pub fn scan(path: PathBuf) -> Result<Project, &'static str> {
    if let Some(root_dir) = path.to_str() {
        let (is_project, config) = match Config::from_dir(&path) {
            Ok(c) => (true, c),
            Err(_) => (false, init::get_default_config(&path)),
        };

        let todos = scan::get_todos(root_dir, &config);

        let project = match todos {
            Ok(tds) => scan::build_project(tds, config),
            Err(_) => return Err("It was not possible to scan the files of the current path."),
        };

        if is_project {
            let mut saved_project = Project::from_dir(&path).unwrap_or(Project::new(String::new(), vec![]));
            let aggregated_project = updater::update_project(&mut saved_project, &project);
            if let Err(_) = export::project_to_path(&aggregated_project, path.clone()) {
                return Err("It was not possible to export project results.");
            };
        };

        Ok(project)
    } else {
        Err("It was not possible to handle given path.")
    }
}

pub fn init(config: Config, path: PathBuf) -> IOResult<()> {
    config.write(&path)?;
    export::init_project_dir(path)?;
    Ok(())
}
