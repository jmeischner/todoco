use std::path::{Path, PathBuf};
use crate::Config;

pub fn get_config_and_project_info_from(path: &PathBuf) -> (bool, Config) {
    match Config::from_dir(&path) {
        Ok(c) => (true, c),
        Err(_) => (false, get_default_config(&path)),
    }
}

pub fn get_default_config(path: &Path) -> Config {
    let cur_dir = match extract_current_directory_name(path) {
        Some(dir) => dir,
        None => "Your Project",
    };

    Config::default(cur_dir)
}

// Todo: There is a problem with extracting current path
fn extract_current_directory_name(path: &Path) -> Option<&str> {
    if let Some(dir) = path.file_name() {
        dir.to_str()
    } else {
        None
    }
}

// ~~~~~~~~~~~~~~~~~~~~ TESTS ~~~~~~~~~~~~~~~~~~~~ //
#[cfg(test)]
mod tests {
    use super::Config;
    use std::path::Path;

    #[test]
    fn get_default_config_with_path() {
        let result = super::get_default_config(Path::new("/here/it/is"));
        let expected = Config::new(String::from("is"), true);

        assert_eq!(result.project.name, expected.project.name);
    }
}