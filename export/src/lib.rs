use appconfig::AppConfig;
use serde_json;
use std::fs;
use std::fs::File;
use std::io::Result as IOResult;
use std::io::Write;
use std::path::PathBuf;
use types::Project;

pub mod format;

mod transition_comparison;

pub fn project_to_path(project: &Project, path: PathBuf) -> IOResult<()> {
    init_project_dir(path.clone())?;
    write_project_to_directory(project, path.clone())?;
    Ok(())
}

pub fn init_project_dir(path: PathBuf) -> IOResult<()> {
    let project_path = get_project_dir_path(path);
    create_project_dir(&project_path)?;
    write_default_todocoignore(project_path.clone())?;
    Ok(())
}

fn get_project_dir_path(mut path: PathBuf) -> PathBuf {
    let appconfig = AppConfig::get();
    path.push(appconfig.names.project_directory.name);
    path
}

fn create_project_dir(path: &PathBuf) -> IOResult<()> {
    fs::create_dir_all(path)?;
    Ok(())
}

fn write_default_todocoignore(mut path: PathBuf) -> IOResult<()> {
    let ignore_filename = AppConfig::get().names.ignore_file;
    path.push(ignore_filename);

    if !path.exists() {
        let mut ignore_file = File::create(path)?;
        ignore_file.write_all(b"**")?;
    };

    Ok(())
}

fn write_project_to_directory(project: &Project, path: PathBuf) -> IOResult<()> {
    let project_filename = AppConfig::get().names.project_directory.project_json;
    let mut path = get_project_dir_path(path);
    path.push(project_filename);

    let project_json = serde_json::to_string(&project)?;

    let mut file = File::create(path)?;
    file.write_all(project_json.as_bytes())?;
    Ok(())
}
