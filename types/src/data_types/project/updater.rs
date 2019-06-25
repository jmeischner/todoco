use crate::{Project, Todo};

pub fn update_project(old: &mut Project, new: &Project) -> Project {
    let active_todos = old.get_active_todos();
    let mut inactive_todos = old.get_inactive_todos();

    let mut result_active_todos = update_todos(active_todos, &mut new.todos.clone());
    result_active_todos.append(&mut inactive_todos);

    Project::new(new.name.clone(), result_active_todos)
}

fn update_todos(old: Vec<&Todo>, new: &mut Vec<Todo>) -> Vec<Todo> {
    let mut result: Vec<Todo> = Vec::new();

    for old_todo in old {
        if let Some(todo) = old_todo.match_in(new) {
            result.push(old_todo.update_with(&todo));
            let index = new.iter().position(|t| t == &todo).unwrap();
            new.remove(index);
        } else {
            let mut inactive_todo = old_todo.clone();
            inactive_todo.set_inactive();
            result.push(inactive_todo);
        }
    }

    result.append(new);

    result
}

// ~~~~~~~~~~~~~~~~~~~~ TESTS ~~~~~~~~~~~~~~~~~~~~ //
#[cfg(test)]
mod tests_updater {
    use crate::{project::updater, Project, SourceFile, Todo};

    #[test]
    fn should_merge_new_todo_to_old_project() {
        let todo1 = Todo::new(
            "You have to this".to_string(),
            SourceFile::new("file 1.txt".to_string(), "in/dir".to_string()),
            2,
            vec![],
        );

        let todo1_new = Todo::new(
            "You have to do this".to_string(),
            SourceFile::new("file 1.txt".to_string(), "in/dir".to_string()),
            2,
            vec![],
        );

        let todo2 = Todo::new(
            "You have to do this! NOW!".to_string(),
            SourceFile::new("file 1.txt".to_string(), "in/another/dir".to_string()),
            56,
            vec![],
        );

        let todo2_new = Todo::new(
            "You have to do this! NOW!".to_string(),
            SourceFile::new("file 1.txt".to_string(), "in/another/dir".to_string()),
            156,
            vec![],
        );

        let old_todo = Todo::new(
            "Just Do".to_string(),
            SourceFile::new("file 1.txt".to_string(), "in/dir".to_string()),
            1,
            vec![],
        );

        let new_todo = Todo::new(
            "Bring me to life".to_string(),
            SourceFile::new("file 1.txt".to_string(), "in/dir".to_string()),
            5,
            vec![],
        );

        let mut old_project = Project::new(
            "Project Old".to_string(),
            vec![todo1.clone(), old_todo.clone(), todo2.clone()],
        );
        let new_project = Project::new(
            "Project New".to_string(),
            vec![todo1_new.clone(), new_todo.clone(), todo2_new.clone()],
        );

        let merged_project = updater::update_project(&mut old_project, &new_project);

        let mut inactive_old_todo = old_todo.clone();
        inactive_old_todo.set_inactive();

        let expected_project = Project::new(
            "Project New".to_string(),
            vec![
                todo1.clone().update_with(&todo1_new),
                inactive_old_todo,
                todo2.clone().update_with(&todo2_new),
                new_todo.clone(),
            ],
        );

        assert_eq!(merged_project, expected_project);
    }
}
