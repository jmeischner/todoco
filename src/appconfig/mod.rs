use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub project_file: String,
}

impl AppConfig {
    pub fn get() -> AppConfig {
        let file = File::open("appconfig.toml").unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).unwrap();
        let result: AppConfig = toml::from_str(&contents).unwrap();

        return result;
    }
}
