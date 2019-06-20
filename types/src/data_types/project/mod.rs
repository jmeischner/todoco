use crate::Todo;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

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
}

// ~~~~~~~~~~~~~~~~~~~~ TESTS ~~~~~~~~~~~~~~~~~~~~ //
#[cfg(test)]
mod tests_mod {

    use crate::{Project, SourceFile, Todo};
    use std::collections::HashMap;

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
}