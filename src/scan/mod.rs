use glob::glob_with;
use glob::MatchOptions;
use glob::{Paths, PatternError};
use regex::Regex;
use std::ffi::OsString;
use std::fs::File;
use std::io::Result as IOResult;
use std::io::{BufRead, BufReader, Lines};

pub mod list;
pub mod project;
pub mod sourcefile;
pub mod todo;

use sourcefile::SourceFile;
use todo::Todo;

fn get_paths_from_dir(dir: &str) -> Result<Paths, PatternError> {
    let options = MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };

    glob_with(dir, options)
}

fn build_file_from_path(paths: Paths) -> Vec<SourceFile> {
    paths
        .filter_map(|glob_res| {
            if let Ok(path) = glob_res {
                if let (Some(name), Some(full_path)) = (path.file_name(), path.to_str()) {
                    Some(SourceFile {
                        name: OsString::from(name),
                        path: OsString::from(full_path),
                    })
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

// Todo: Parallel Extraction
fn extract_todos_from_files(files: Vec<SourceFile>) -> IOResult<Vec<Todo>> {
    let mut todos: Vec<Todo> = vec![];

    for file in files {
        let content = read_lines_of_file(&file)?;
        let fileTodos = extract_todos_from_content(content, file);
        todos.extend(fileTodos);
    }

    Ok(todos)
}

fn read_lines_of_file(file: &SourceFile) -> IOResult<Lines<BufReader<File>>> {
    let f = File::open(*file.path.to_str())?;
    let reader = BufReader::new(f).lines();
    Ok(reader)
}

fn extract_todos_from_content(lines: Lines<BufReader<File>>, file: SourceFile) -> Vec<Todo> {
    let mut todos: Vec<Todo> = vec![];
    let todo_regex = Regex::new(r"(?i:todo+:?\s?)(?P<todo>.*$)").unwrap();

    for (lnr, line) in lines.enumerate() {
        if let Some(todo) = todo_regex.find(&line.unwrap()) {
            todos.push(Todo {
                text: String::from(todo.as_str()),
                line: lnr,
                file: file,
            })
        }
    }

    todos
}

// ~~~~~~~~~~~~~~~~~~~~ TESTS ~~~~~~~~~~~~~~~~~~~~ //
#[cfg(test)]
mod tests {
    use super::SourceFile;
    use std::ffi::OsString;
    use std::path::PathBuf;

    #[test]
    fn find_paths_from_dir() {
        for p in super::get_paths_from_dir("env_tests/mod_scan/*.txt").unwrap() {
            if let Ok(path) = p {
                let mut expected_path = PathBuf::new();
                expected_path.push("env_tests");
                expected_path.push("mod_scan");
                expected_path.push("file1");
                expected_path.set_extension("txt");
                assert_eq!(path, expected_path)
            }
        }
    }

    #[test]
    fn create_file_vec_from_path() {
        let test_path = super::get_paths_from_dir("env_tests/mod_scan/*.txt").unwrap();
        let files = super::build_file_from_path(test_path);
        let expected = vec![SourceFile {
            name: OsString::from("file1.txt"),
            path: OsString::from("env_tests/mod_scan/file1.txt"),
        }];
        assert_eq!(files, expected)
    }
}
