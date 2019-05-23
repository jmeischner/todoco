use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub names: Names,
}

impl AppConfig {
    pub fn get() -> AppConfig {
        let file = include_str!("../../appconfig.toml");
        let result: AppConfig = toml::from_str(file).unwrap();

        return result;
    }
}

#[derive(Serialize, Deserialize)]
pub struct Names {
    pub project_file: String,
    pub ignore_file: String,
    pub project_directory: String,
}
