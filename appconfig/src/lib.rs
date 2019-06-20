use serde::{Deserialize, Serialize};
use toml;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub names: Names,
}

impl AppConfig {
    pub fn get() -> AppConfig {
        let file = include_str!("../appconfig.toml");
        let result: AppConfig = toml::from_str(file).unwrap();

        return result;
    }

    pub fn get_project_dir_path(&self, mut path: PathBuf) -> PathBuf {
    path.push(&self.names.project_directory.name);
    path
}
}

#[derive(Serialize, Deserialize)]
pub struct Names {
    pub project_file: String,
    pub ignore_file: String,
    pub project_directory: ProjectDirectory,
}

#[derive(Serialize, Deserialize)]
pub struct ProjectDirectory {
    pub name: String,
    pub project_json: String,
    pub export_taskpaper_extension: String,
}
