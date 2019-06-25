use types::{FilterMatch, Project, SourceFile, Tag, Todo};

use log::warn;
use std::path::{Path, PathBuf};


pub fn build_current_dir_path() -> PathBuf {
    let mut current_dir_path = PathBuf::new();
    current_dir_path.push(".");
    current_dir_path
}

pub fn get_project(is_project: bool, path: &Path) -> Result<Project, &'static str> {
    if is_project {
        match Project::from_dir(path) {
            Some(project) => Ok(project),
            None => {
                warn!("Could not read saved project information, rescan project!");
                crate::scan(path.to_path_buf())
            }
        }
    } else {
        crate::scan(path.to_path_buf())
    }
}

pub fn get_matching_todos<'a>(keyword: &'a str, project: &Project) -> FilterMatch {
    if let Some(matching_tags) = check_for_keyword_in_tags(keyword, project) {
        let todos = matching_tags
            .iter()
            .flat_map(|tag| project.get_todos_with_tag(tag))
            .map(|todo| todo.clone())
            .collect();
        return FilterMatch::Tags(todos);
    }

    if let Some(matching_files) = check_for_keyword_in_filenames(keyword, project) {
        let todos = matching_files
            .iter()
            .flat_map(|file| project.get_todos_in_file(file))
            .map(|todo| todo.clone())
            .collect();
        return FilterMatch::Files(todos);
    }

    if let Some(matching_todos) = check_for_keyword_in_todos(keyword, project) {
        let todos = matching_todos.iter().map(|&todo| todo.clone()).collect();
        return FilterMatch::Text(todos);
    }


    FilterMatch::None
}


fn check_for_keyword_in_tags<'a>(keyword: &'a str, project: &'a Project) -> Option<Vec<&'a Tag>> {
    let tags = project.get_tags();
    let matching_tags: Vec<&Tag> = tags
        .iter()
        .cloned()
        .filter(|tag| tag.name.contains(keyword))
        .collect();

    match matching_tags.len() {
        0 => None,
        _ => Some(matching_tags),
    }
}

fn check_for_keyword_in_filenames<'a>(
    keyword: &'a str,
    project: &'a Project,
) -> Option<Vec<&'a SourceFile>> {
    let files = project.get_files();
    let matching_files: Vec<&SourceFile> = files
        .iter()
        .cloned()
        .filter(|file| file.name.contains(keyword))
        .collect();

    match matching_files.len() {
        0 => None,
        _ => Some(matching_files),
    }
}

fn check_for_keyword_in_todos<'a>(keyword: &'a str, project: &'a Project) -> Option<Vec<&'a Todo>> {
    let matching_todos: Vec<&Todo> = project
        .todos
        .iter()
        .filter(|todo| todo.text.contains(keyword))
        .collect();

    match matching_todos.len() {
        0 => None,
        _ => Some(matching_todos),
    }
}