use std::io::Result as IOResult;
use std::path::PathBuf;
use todofilter;
use types::{Config, FilterMatch, Project};

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
    todofilter::get_filtered_todos(keyword, current_dir)
}