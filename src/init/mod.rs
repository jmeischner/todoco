use config::Config;
use std::path::Path;

pub mod config;
pub mod project;

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
        let expected = Config::new(String::from("is"));

        assert_eq!(result.name, expected.name);
    }
}
