use dialoguer::Input;
use std::io::Result as IOResult;

// Todo: Use Theme
pub fn ask_for_config() {
    // Todo: Default value should be directory name
    let project_result: IOResult<String> = Input::new().with_prompt("project name").interact();

    if let Ok(p) = project_result {
        println!("{}", p);
    } else {
        println!("Please answer the question:");
        ask_for_config();
    }
}
