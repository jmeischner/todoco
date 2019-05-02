use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::io::{Error as IOError, ErrorKind, Result as IOResult};
use std::path::Path;
use toml;

use crate::appconfig::AppConfig;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub name: String,
}

impl Config {
    pub fn new(name: String) -> Config {
        Config { name: name }
    }

    pub fn get_from_dir(path: &Path) -> IOResult<Config> {
        let config = AppConfig::get();
        let mut path = path.to_path_buf();
        path.push(config.project_file);
        let file = File::open(path)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;
        let result: Result<Config, toml::de::Error> = toml::from_str(&contents);

        match result {
            Ok(config) => Ok(config),
            Err(_) => Err(IOError::new(
                ErrorKind::InvalidData,
                "Config file contains invalid Toml data.",
            )),
        }
    }

    pub fn write(&self, path: &Path) -> IOResult<()> {
        let config_text =
            toml::to_string_pretty(&self).expect("It was not possible to serialize configuration.");

        let config = AppConfig::get();
        let mut path = path.to_path_buf();
        path.push(config.project_file);
        let mut file = File::create(path)?;

        file.write_all(config_text.as_bytes())?;

        Ok(())
    }
}

// ~~~~~~~~~~~~~~~~~~~~ TESTS ~~~~~~~~~~~~~~~~~~~~ //
#[cfg(test)]
mod tests {
    use super::Config;
    use std::path::Path;

    #[test]
    fn get_config_from_file() {
        let config = Config::get_from_dir(Path::new("env_tests/mod_init")).unwrap();
        let expected = Config::new(String::from("test"));
        assert_eq!(config.name, expected.name);
    }
}
