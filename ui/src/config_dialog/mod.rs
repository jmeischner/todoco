use dialoguer::{Confirmation, Input};
use std::env;
use std::io::Result as IOResult;
use types::Config;

// Todo: Use dialoguer Theme
pub fn ask_for_config() -> IOResult<Config> {
    println!("This utility will walk you through creating a todoco.toml file.");

    let p_name = get_project_name()?;
    let p_use_gitignore = get_use_gitignore()?;

    Ok(Config::new(p_name, p_use_gitignore))
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

fn get_project_name() -> IOResult<String> {
    let mut project_input = Input::new();

    if let Some(dir) = extract_current_dir_from_env()? {
        project_input.default(dir);
    }

    // Todo: Use appconfig for prompt
    project_input.with_prompt("project name");

    Ok(project_input.interact()?)
}

fn get_use_gitignore() -> IOResult<bool> {
    // Todo: Use appconfig for prompt
    let mut use_gitignore_input = Confirmation::new();

    use_gitignore_input
        .default(true)
        .with_text("respect .gitignore?");

    Ok(use_gitignore_input.interact()?)
}
