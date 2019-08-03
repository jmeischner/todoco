use crate::{Project, Todo};

/// Merges the old project information with
/// the newly scanned informations
///
/// ### Arguments
/// - `old`: Project read from .todoco directory
/// - `new`: Project scanned from current files in project
///
/// Returns merged project
pub fn update_project(old: &mut Project, new: &Project) -> Project {
    let active_todos = old.get_todos().to_vec();
    let (actives, mut inactives) = update_todos(active_todos, &mut new.todos.clone());

    let mut project = Project::new(&new.name, actives);
    project.append_archive(&mut inactives);
    project
}

/// Checks which Todos are new, which already exists and which don't exist anymore
///
/// ### Arguments
/// - `old`: Todos from old Project
/// - `new`: Todos from new Project
///
/// ### Return
/// 1. List of todos which are new or already exist
/// 2. List of todos which were deleted between last scans
fn update_todos(old: Vec<Todo>, new: &mut Vec<Todo>) -> (Vec<Todo>, Vec<Todo>) {
    let mut active_todos: Vec<Todo> = Vec::new();
    let mut inactive_todos: Vec<Todo> = Vec::new();

    for old_todo in old {
        if let Some(todo) = old_todo.match_in(new) {
            active_todos.push(old_todo.update_with(&todo));
            let index = new.iter().position(|t| t == &todo).unwrap();
            new.remove(index);
        } else {
            let mut inactive_todo = old_todo.clone();
            inactive_todo.set_inactive();
            inactive_todos.push(inactive_todo);
        }
    }

    active_todos.append(new);

    (active_todos, inactive_todos)
}

// ~~~~~~~~~~~~~~~~~~~~ TESTS ~~~~~~~~~~~~~~~~~~~~ //
#[cfg(test)]
mod tests_updater {
    use crate::{project::updater, Project, SourceFile, Todo};

    #[test]
    fn should_merge_new_todo_to_old_project() {
        let todo1 = Todo::new(
            "You have to this",
            SourceFile::new("file 1.txt", "in/dir"),
            2,
            vec![],
        );

        let todo1_new = Todo::new(
            "You have to do this",
            SourceFile::new("file 1.txt", "in/dir"),
            2,
            vec![],
        );

        let todo2 = Todo::new(
            "You have to do this! NOW!",
            SourceFile::new("file 1.txt", "in/another/dir"),
            56,
            vec![],
        );

        let todo2_new = Todo::new(
            "You have to do this! NOW!",
            SourceFile::new("file 1.txt", "in/another/dir"),
            156,
            vec![],
        );

        let old_todo = Todo::new(
            "Just Do",
            SourceFile::new("file 1.txt", "in/dir"),
            1,
            vec![],
        );

        let new_todo = Todo::new(
            "Bring me to life",
            SourceFile::new("file 1.txt", "in/dir"),
            5,
            vec![],
        );

        let mut old_project = Project::new(
            "Project Old",
            vec![todo1.clone(), old_todo.clone(), todo2.clone()],
        );
        let new_project = Project::new(
            "Project New",
            vec![todo1_new.clone(), new_todo.clone(), todo2_new.clone()],
        );

        let merged_project = updater::update_project(&mut old_project, &new_project);

        let mut inactive_old_todo = old_todo.clone();
        inactive_old_todo.set_inactive();
        let mut archive = vec![inactive_old_todo];

        let mut expected_project = Project::new(
            "Project New",
            vec![
                todo1.clone().update_with(&todo1_new),
                todo2.clone().update_with(&todo2_new),
                new_todo.clone(),
            ],
        );

        expected_project.append_archive(&mut archive);

        assert_eq!(merged_project, expected_project);
    }
}
