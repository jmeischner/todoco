use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub project_file: String,
}

impl AppConfig {
    pub fn get() -> AppConfig {
        let file = include_str!("../../appconfig.toml");
        let result: AppConfig = toml::from_str(file).unwrap();

        return result;
    }
}
