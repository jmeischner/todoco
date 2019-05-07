use std::io::Result as IOResult;
use std::path::PathBuf;

pub mod appconfig;
pub mod init;
pub mod scan;

pub use init::config::Config;
pub use scan::list::List;
pub use scan::project::Project;
pub use scan::sourcefile::SourceFile;
pub use scan::todo::Todo;

// Todo: add error propagation
pub fn scan(path: PathBuf) -> Result<Project, &'static str> {
    if let Some(root_dir) = path.to_str() {
        let config = match Config::get_from_dir(&path) {
            Ok(c) => c,
            Err(_) => init::get_default_config(&path),
        };

        let files = scan::get_files(root_dir, &config);
        let todos = scan::extract_todos_from_files(files);
        Ok(scan::build_project(todos.unwrap(), config))
    } else {
        Err("It was not possible to handle given path.")
    }
}

pub fn init(config: Config, path: PathBuf) -> IOResult<()> {
    config.write(&path)?;
    Ok(())
}
