use dialoguer::Input;
use std::io::Result as IOResult;
use todoco::Config;

// Todo: Use dialoguer Theme
pub fn ask_for_config() -> IOResult<Config> {
    println!("This utility will walk you through creating a todoco.toml file.");

    // Todo: Default value should be directory name
    let project_name = Input::new().with_prompt("project name").interact()?;

    Ok(Config::new(project_name))
}
