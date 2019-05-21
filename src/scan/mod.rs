use ignore::{Walk, WalkBuilder};
use std::fs::File;
use std::io::Result as IOResult;
use std::io::{BufRead, BufReader, Lines};

use crate::Config;
use list::List;
use project::Project;
use sourcefile::SourceFile;
use todo::Todo;

use crate::appconfig::AppConfig;

pub mod list;
pub mod project;
pub mod sourcefile;
pub mod todo;
mod todo_regex;

pub fn get_files(dir: &str, config: &Config) -> Vec<SourceFile> {
    // Todo: If no files were found, then give user output
    let paths = get_path_walker_from_dir(dir, config);
    build_file_from_path(paths)
}

// Todo: Parallel Extraction
pub fn extract_todos_from_files(files: Vec<SourceFile>) -> IOResult<Vec<Todo>> {
    let mut todos: Vec<Todo> = vec![];

    for file in files {
        let lines = read_lines_of_file(&file)?;
        let file_todos = extract_todos_from_content(lines, file);
        todos.extend(file_todos);
    }

    Ok(todos)
}

pub fn build_project(todos: Vec<Todo>, config: Config) -> Project {
    let list = List::new(String::from("All"), todos);
    let lists = vec![list];
    Project::new(config.project.name, lists)
}

fn get_path_walker_from_dir(dir: &str, config: &Config) -> Walk {
    let mut walker = WalkBuilder::new(dir);
    walker.git_ignore(config.project.use_gitignore);
    walker.add_custom_ignore_filename(AppConfig::get().names.ignore_file);
    walker.build()
}

fn build_file_from_path(paths: Walk) -> Vec<SourceFile> {
    paths
        .filter_map(|direntry| {
            if let Ok(entry) = direntry {
                let path = entry.path();
                if path.is_file() {
                    if let (Some(name), Some(full_path)) = (path.file_name(), path.to_str()) {
                        if let Some(filename) = name.to_str() {
                            Some(SourceFile::new(
                                String::from(filename),
                                String::from(full_path),
                            ))
                        } else {
                            // Todo: handle Result appropriatly
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                // Todo: Handle GlobError appropriately
                None
            }
        })
        .collect()
}

fn read_lines_of_file(file: &SourceFile) -> IOResult<Lines<BufReader<File>>> {
    let f = File::open(&file.path)?;
    let reader = BufReader::new(f).lines();
    Ok(reader)
}

fn extract_todos_from_content(lines: Lines<BufReader<File>>, file: SourceFile) -> Vec<Todo> {
    let mut todos: Vec<Todo> = vec![];
    // Todo: Add configurable todo regex
    // Todo: Add regex for all kind of comment begins @bla(bli) @blo(blu)

    for (lnr, line) in lines.enumerate() {
        match line {
            Ok(l) => {
                // if todo_regex::is_todo(&l) {
                //     let matches = todo_regex.matches(&l);
                // }
                // for capture in todo_regex.is_match(&l) {
                //     // let found_todo =
                //     //     Todo::new(String::from(&capture["todo"]), file.clone(), lnr + 1);
                //     let found_todo = Todo::new(String::from("b"), file.clone(), lnr + 1);
                //     println!("{:?}", &capture);
                //     println!("{:?}", todo_regex.patterns()[capture]);
                //     todos.push(found_todo)
                // }
            }
            // Todo: handle not valid utf-8 appropriatly
            Err(_e) => break,
        };
    }

    todos
}

// ~~~~~~~~~~~~~~~~~~~~ TESTS ~~~~~~~~~~~~~~~~~~~~ //
#[cfg(test)]
mod tests {
    use super::SourceFile;
    use super::Todo;
    use crate::Config;
    use std::path::PathBuf;

    #[test]
    fn find_paths_from_dir() {
        let path = "env_tests/mod_scan/";
        for p in super::get_path_walker_from_dir(path, &Config::default(path)).skip(1) {
            if let Ok(path) = p {
                let mut expected_path = PathBuf::new();
                expected_path.push("env_tests");
                expected_path.push("mod_scan");
                expected_path.push("file1");
                expected_path.set_extension("txt");
                assert_eq!(path.path(), expected_path)
            }
        }
    }

    #[test]
    fn create_file_vec_from_path() {
        let path = "env_tests/mod_scan/";
        let test_path = super::get_path_walker_from_dir(path, &Config::default(path));
        let files = super::build_file_from_path(test_path);
        let expected = vec![SourceFile {
            name: String::from("file1.txt"),
            path: String::from("env_tests/mod_scan/file1.txt"),
        }];
        assert_eq!(files, expected)
    }

    #[test]
    fn extract_todo_from_test_file() {
        let test_file = SourceFile::new(
            String::from("file1.txt"),
            String::from("env_tests/mod_scan/file1.txt"),
        );
        let expected_todo1 = Todo::new(String::from("Test"), test_file.clone(), 1);
        let expected_todo2 = Todo::new(String::from("Test"), test_file.clone(), 2);

        let todo = super::extract_todos_from_files(vec![test_file]).unwrap();

        assert_eq!(todo, vec![expected_todo1, expected_todo2])
    }
}
