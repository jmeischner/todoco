use dialoguer::Input;
use std::env;
use std::io::Result as IOResult;
use todoco::Config;

// Todo: Use dialoguer Theme
pub fn ask_for_config() -> IOResult<Config> {
    println!("This utility will walk you through creating a todoco.toml file.");

    let mut project_input = Input::new();

    if let Some(dir) = extract_current_dir_from_env()? {
        project_input.default(dir);
    }

    project_input.with_prompt("project name");

    let project_name = project_input.interact()?;

    Ok(Config::new(project_name))
}

fn extract_current_dir_from_env() -> IOResult<Option<String>> {
    if let Some(dir) = env::current_dir()?.file_name() {
        if let Some(string) = dir.to_str() {
            Ok(Some(String::from(string)))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}
