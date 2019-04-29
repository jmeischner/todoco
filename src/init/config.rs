use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::io::{Error as IOError, ErrorKind, Result as IOResult};
use std::path::Path;
use toml;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub name: String,
}

impl Config {
    pub fn new(name: String) -> Config {
        Config { name: name }
    }

    pub fn get_from_dir(path: &Path) -> IOResult<Config> {
        // Todo: get config file name from app.config
        let mut path = path.to_path_buf();
        path.push("todoco.toml");
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

        // Todo: get filename from app.config
        let mut path = path.to_path_buf();
        path.push("todoco.toml");
        let mut file = File::create(path)?;

        file.write_all(config_text.as_bytes())?;

        Ok(())
    }
}

// ~~~~~~~~~~~~~~~~~~~~ TESTS ~~~~~~~~~~~~~~~~~~~~ //
#[cfg(test)]
mod tests {
    use super::Config;
    use std::fs::{remove_file, File};
    use std::io::Read;
    use std::path::Path;

    #[test]
    fn write_config_test() {
        let config = Config::new(String::from("bla"));
        config.write(&Path::new("."));
        let mut content = String::new();
        File::open("todoco.toml")
            .unwrap()
            .read_to_string(&mut content);
        let expected = "name = \'bla\'\n";
        assert_eq!(content, expected);
        remove_file("todoco.toml");
    }

    #[test]
    fn get_config_from_file() {
        let config = Config::get_from_dir(Path::new("env_tests/mod_init")).unwrap();
        let expected = Config::new(String::from("test"));
        assert_eq!(config.name, expected.name);
    }
}
