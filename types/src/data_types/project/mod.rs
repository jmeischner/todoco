use crate::{SourceFile, Tag, Todo};
use appconfig::AppConfig;
use itertools::Itertools;
use log::error;

use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;


pub mod updater;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Project {
    pub name: String,
    pub todos: Vec<Todo>,
    archive: Option<Vec<Todo>>,
}

/// Constructor, Getter and Setter
impl Project {
    pub fn new(name: &str, todos: Vec<Todo>) -> Project {
        Project {
            name: name.to_string(),
            todos: todos,
            archive: None,
        }
    }

    pub fn set_archive(&mut self, archive: Vec<Todo>) {
        self.archive = Some(archive);
    }

    pub fn append_archive(&mut self, newly_archived: &mut Vec<Todo>) {
        if let Some(ref mut archive) = self.archive {
            archive.append(newly_archived);
        } else {
            self.set_archive(newly_archived.clone());
        }
    }

    pub fn get_todos(&self) -> &Vec<Todo> {
        &self.todos
    }

    pub fn get_archive(&self) -> Vec<Todo> {
        if let Some(archive) = &self.archive {
            archive.clone()
        } else {
            vec![]
        }
    }
}

/// Other Functions for Project
impl Project {
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

    pub fn get_tags(&self) -> Vec<&Tag> {
        self.todos
            .iter()
            .flat_map(|todo| &todo.tags)
            .unique_by(|tag| &tag.name)
            .collect()
    }

    pub fn get_files(&self) -> Vec<&SourceFile> {
        self.todos
            .iter()
            .map(|todo| &todo.file)
            .unique_by(|file| format!("{}/{}", &file.path, &file.name))
            .collect()
    }

    pub fn get_todos_with_tag<'a>(&'a self, tag: &'a Tag) -> Vec<&'a Todo> {
        self.todos
            .iter()
            .filter(|todo| todo.tags.iter().find(|t| t == &tag).is_some())
            .collect()
    }

    pub fn get_todos_in_file(&self, file: &SourceFile) -> Vec<&Todo> {
        self.todos
            .iter()
            .filter(|todo| &todo.file == file)
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
        let file1 = SourceFile::new("File 1.txt", "Here/it/is");
        let file2 = SourceFile::new("File 2.txt", "Here/it/is");
        let td1 = Todo::new("Todo 1", file1.clone(), 1, vec![]);
        let td2 = Todo::new("Todo 2", file1.clone(), 1, vec![]);
        let td3 = Todo::new("Todo 3", file1.clone(), 1, vec![]);
        let td4 = Todo::new("Todo 4", file2.clone(), 1, vec![]);

        let project = Project::new(
            "My List",
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
        let project = Project::new("My List", vec![]);
        let grouped = project.group_by_file();
        assert_eq!(grouped, None);
    }

    #[test]
    fn could_read_project_file() {
        let todo1 = Todo::new(
            "Use dialoguer Theme",
            SourceFile::new("dialog_config.rs", "./ui/src"),
            6,
            vec![],
        );
        let mut todo2 = Todo::new("Old Todo", SourceFile::new("mod.rs", "./ui/src"), 6, vec![]);
        todo2.set_inactive();

        let project = Project::from_dir(Path::new("fixtures/project")).unwrap();
        let expected_project = Project::new("todoco", vec![todo1, todo2]);

        assert_eq!(expected_project.name, project.name);
        assert_eq!(expected_project.todos.len(), project.todos.len());
    }

    #[test]
    fn should_append_archive_to_existing_project() {
        let todo1 = Todo::new(
            "Some Meaningful Text",
            SourceFile::new("Here", "test.rs"),
            1,
            vec![],
        );
        let mut project = Project::new("Test Project", vec![todo1]);

        let mut todo2 = Todo::new(
            "Some Other Text",
            SourceFile::new("Here", "test.rs"),
            1,
            vec![],
        );
        todo2.set_inactive();

        let mut archive = vec![todo2];
        project.append_archive(&mut archive);

        assert_eq!(project.get_archive().len(), 1);

        let mut todo3 = Todo::new(
            "Some Meaningful Other Text",
            SourceFile::new("Here", "test.rs"),
            1,
            vec![],
        );
        todo3.set_inactive();

        let mut archive = vec![todo3];
        project.append_archive(&mut archive);

        assert_eq!(project.get_archive().len(), 2);
    }
}
