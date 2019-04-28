use std::io::Result as IOResult;

pub mod init;
pub mod scan;

pub use init::config::Config;
pub use scan::list::List;
pub use scan::project::Project;
pub use scan::sourcefile::SourceFile;
pub use scan::todo::Todo;

// Todo: add error propagation
pub fn scan(root_dir: &str) -> Project {
    let glob = scan::map_dir_to_glob(root_dir);
    let files = scan::get_files(&glob);
    let todos = scan::extract_todos_from_files(files.unwrap());
    scan::build_project(todos.unwrap())
}

pub fn init(config: Config) -> IOResult<()> {
    config.write()?;
    Ok(())
}
