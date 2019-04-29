use config::Config;
use std::path::Path;

pub mod config;

pub fn get_default_config(path: &Path) -> Config {
    let cur_dir = match extract_current_directory_name(path) {
        Some(dir) => dir,
        None => "Your Project",
    };

    Config::new(String::from(cur_dir))
}

fn extract_current_directory_name(path: &Path) -> Option<&str> {
    if let Some(dir) = path.file_name() {
        dir.to_str()
    } else {
        None
    }
}
