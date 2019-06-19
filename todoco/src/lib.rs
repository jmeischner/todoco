use std::io::Result as IOResult;
use std::path::PathBuf;
use types::{Project, Config};

pub mod init;
pub mod scan;

// Todo: add error propagation
pub fn scan(path: PathBuf) -> Result<Project, &'static str> {
    if let Some(root_dir) = path.to_str() {
        let (is_project, config) = match Config::get_from_dir(&path) {
            Ok(c) => (true, c),
            Err(_) => (false, init::get_default_config(&path)),
        };

        let files = scan::get_files(root_dir, &config);
        let todos = scan::extract_todos_from_files(files);

        let project = match todos {
            Ok(tds) => scan::build_project(tds, config),
            Err(_) => return Err("It was not possible to scan the files of the current path."),
        };

        if is_project {
            if let Err(_) = export::project_to_path(&project, path.clone()) {
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
