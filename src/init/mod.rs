use config::Config;
use std::path::Path;

pub mod config;

pub fn get_default_config(path: &Path) -> Config {
    let given_dir = if let Some(dir) = path.components().last() {
        if let Some(d) = dir.as_os_str().to_str() {
            d
        } else {
            "Your Project"
        }
    } else {
        "Your Project"
    };

    Config::new(String::from(given_dir))
}
