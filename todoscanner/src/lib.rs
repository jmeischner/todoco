use export;
use std::path::PathBuf;
use types::{config::helper, project::updater, Project};
pub mod scan;

pub fn scan(path: PathBuf) -> Result<Project, &'static str> {
    if let Some(root_dir) = path.to_str() {
        let (is_project, config) = helper::get_config_and_project_info_from(&path);

        let todos = scan::get_todos(root_dir, &config);

        let project = match todos {
            Ok(tds) => scan::build_project(tds, config),
            Err(_) => return Err("It was not possible to scan the files of the current path."),
        };

        if is_project {
            let mut saved_project = scan::get_saved_project(&path);
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
