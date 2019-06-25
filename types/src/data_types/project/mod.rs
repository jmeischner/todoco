use crate::Todo;
use appconfig::AppConfig;
use log::error;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

pub mod updater;

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub name: String,
    pub todos: Vec<Todo>,
}

impl Project {
    pub fn new(name: String, todos: Vec<Todo>) -> Project {
        Project {
            name: name,
            todos: todos,
        }
    }

    pub fn from_dir(path: &Path) -> Option<Project> {
        let config = AppConfig::get();
        let mut path = config.get_project_dir_path(path.to_path_buf());
        path.push(config.names.project_directory.project_json);
        if let Ok(file) = File::open(path) {
            let mut buf_reader = BufReader::new(file);
            let mut contents = String::new();

            if let Ok(_) = buf_reader.read_to_string(&mut contents) {
                match serde_json::from_str(&contents) {
                    Ok(result) => return Some(result),
                    Err(e) => error!("Error while deserializing project.json: {}", e),
                }
            };
        }

        None
    }

    pub fn group_by_file(&self) -> Option<HashMap<String, Vec<&Todo>>> {
        if self.todos.len() == 0 {
            return None;
        }

        let mut map: HashMap<String, Vec<&Todo>> = HashMap::new();

        for todo in &self.todos {
            let full_file_path = format!("{}/{}", todo.file.path, todo.file.name);
            let mut current = map.get(&full_file_path).unwrap_or(&vec![]).clone();
            current.push(&todo);
            map.insert(full_file_path, current.to_vec());
        }

        return Some(map);
    }

    pub fn get_active_todos(&self) -> Vec<&Todo> {
        self.todos.iter().filter(|todo| todo.is_active).collect()
    }

    pub fn get_inactive_todos(&self) -> Vec<Todo> {
        self.todos
            .iter()
            .cloned()
            .filter(|todo| !todo.is_active)
            .collect()
    }
}

impl PartialEq for Project {
    fn eq(&self, other: &Project) -> bool {
        self.name == other.name && self.todos == other.todos
    }
}

// ~~~~~~~~~~~~~~~~~~~~ TESTS ~~~~~~~~~~~~~~~~~~~~ //
#[cfg(test)]
mod tests_mod {

    use crate::{Project, SourceFile, Todo};
    use std::collections::HashMap;
    use std::path::Path;

    #[test]
    fn get_todos_grouped_by_files() {
        let file1 = SourceFile::new(String::from("File 1.txt"), String::from("Here/it/is"));
        let file2 = SourceFile::new(String::from("File 2.txt"), String::from("Here/it/is"));
        let td1 = Todo::new(String::from("Todo 1"), file1.clone(), 1, vec![]);
        let td2 = Todo::new(String::from("Todo 2"), file1.clone(), 1, vec![]);
        let td3 = Todo::new(String::from("Todo 3"), file1.clone(), 1, vec![]);
        let td4 = Todo::new(String::from("Todo 4"), file2.clone(), 1, vec![]);

        let project = Project::new(
            String::from("My List"),
            vec![td1.clone(), td2.clone(), td3.clone(), td4.clone()],
        );

        let ordered_list = project.group_by_file();

        let mut expected_result = HashMap::new();
        expected_result.insert("Here/it/is/File 1.txt".to_string(), vec![&td1, &td2, &td3]);
        expected_result.insert("Here/it/is/File 2.txt".to_string(), vec![&td4]);

        assert_eq!(ordered_list, Some(expected_result));
    }

    #[test]
    fn try_to_group_todos_of_empty_list() {
        let project = Project::new("My List".to_string(), vec![]);
        let grouped = project.group_by_file();
        assert_eq!(grouped, None);
    }

    #[test]
    fn could_read_project_file() {
        let project = Project::from_dir(Path::new("fixtures/project")).unwrap();
        let expected_project = Project::new(
            "todoco".to_string(),
            vec![Todo::new(
                "Use dialoguer Theme".to_string(),
                SourceFile::new(
                    "dialog_config.rs".to_string(),
                    "./ui/src/dialog_config.rs".to_string(),
                ),
                6,
                vec![],
            )],
        );

        assert_eq!(expected_project.name, project.name);
        assert_eq!(expected_project.todos.len(), project.todos.len());
    }
}
