use serde::{Deserialize, Serialize};

use std::path::PathBuf;
use toml;
#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub names: Names,
    pub default_values: DefaultValues,
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

#[derive(Serialize, Deserialize)]
pub struct DefaultValues {
    pub ignores: Vec<String>,
}