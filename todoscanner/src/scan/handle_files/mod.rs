use ignore::{
    overrides::{Override, OverrideBuilder},
    Walk, WalkBuilder,
};
use indicatif::{ProgressBar, ProgressStyle};
use log::debug;
use std::fs::File;
use std::io::Result as IOResult;
use std::io::{BufRead, BufReader, Lines};

use appconfig::AppConfig;
use todo_regex::TodoRegexer;
use types::{Config, SourceFile, Todo};

mod todo_regex;

pub fn get_files(dir: &str, config: &Config) -> Vec<SourceFile> {
    // Todo: If no files were found, then give user output
    let paths = get_path_walker_from_dir(dir, config);
    build_file_from_path(paths)
}

// Todo: Parallel Extraction
pub fn extract_todos_from_files(files: Vec<SourceFile>) -> IOResult<Vec<Todo>> {
    let mut todos: Vec<Todo> = vec![];
    let bar = build_progress_bar(files.len() as u64);

    for (index, file) in files.iter().enumerate() {
        let lines = read_lines_of_file(&file)?;
        let file_todos = extract_todos_from_content(lines, file);
        todos.extend(file_todos);
        if (index + 1) % 5 == 0 {
            bar.inc(5);
        }
    }

    bar.finish();
    Ok(todos)
}
fn build_progress_bar(max: u64) -> ProgressBar {
    let bar = ProgressBar::new(max);
    bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed:.cyan/blue}] |{wide_bar:.cyan/blue}| {pos:>7.cyan/blue} of {len:7.cyan/blue}")
            .progress_chars("=>-"),
    );

    bar
}

fn get_path_walker_from_dir(dir: &str, config: &Config) -> Walk {
    debug!("Used config for WalkBuilder: {:?}", config);

    let default_overrides = build_default_overrides(dir);
    let mut walker = WalkBuilder::new(dir);
    walker.ignore(config.project.use_ignore);
    walker.add_custom_ignore_filename(AppConfig::get().names.ignore_file);
    walker.git_ignore(config.project.use_gitignore);
    walker.hidden(config.project.search_hidden);
    walker.overrides(default_overrides);
    debug!("Used walker: {:?}", walker);

    walker.build()
}

fn build_default_overrides(path: &str) -> Override {
    let defaults = AppConfig::get().default_values.ignores;

    let mut ignores = OverrideBuilder::new(path);

    for ignore in &defaults {
        ignores.add(ignore).unwrap();
    }

    ignores.build().unwrap()
}

fn build_file_from_path(paths: Walk) -> Vec<SourceFile> {
    paths
        .filter_map(|direntry| {
            if let Ok(entry) = direntry {
                let path = entry.path();
                if path.is_file() {
                    // Todo: What if one of the 'Some's fails
                    if let (Some(name), Some(full_path)) = (path.file_name(), path.to_str()) {
                        if let Some(filename) = name.to_str() {
                            Some(SourceFile::new(
                                filename,
                                full_path,
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

fn extract_todos_from_content(lines: Lines<BufReader<File>>, file: &SourceFile) -> Vec<Todo> {
    let mut todos: Vec<Todo> = vec![];
    let todo_regexer = TodoRegexer::new();

    for (lnr, line) in lines.enumerate() {
        match line {
            Ok(l) => {
                if todo_regexer.is_todo(&l) {
                    if let Some(todo_text) = todo_regexer.extract_text(&l) {
                        let line = lnr + 1;
                        let file = file.clone();
                        let (text, tags) = todo_regexer.extract_tags_from_text(todo_text);

                        let todo = Todo::new(text, file, line, tags);
                        todos.push(todo)
                    }
                }
            }
            // Todo: handle not valid utf-8 appropriatly
            Err(_e) => break,
        }
    }

    todos
}

// ~~~~~~~~~~~~~~~~~~~~ TESTS ~~~~~~~~~~~~~~~~~~~~ //
#[cfg(test)]
mod tests {

    use std::path::PathBuf;
    use types::Config;
    use types::{SourceFile, Tag, Todo};
    #[test]
    fn find_paths_from_dir() {
        let path = "fixtures/mod_scan/";
        for p in super::get_path_walker_from_dir(path, &Config::default(path)).skip(1) {
            if let Ok(path) = p {
                let mut expected_path = PathBuf::new();
                expected_path.push("fixtures");
                expected_path.push("mod_scan");
                expected_path.push("file1");
                expected_path.set_extension("txt");
                assert_eq!(path.path(), expected_path)
            }
        }
    }

    #[test]
    fn create_file_vec_from_path() {
        let path = "fixtures/mod_scan/";
        let test_path = super::get_path_walker_from_dir(path, &Config::default(path));
        let files = super::build_file_from_path(test_path);
        let expected = vec![SourceFile {
            name: String::from("file1.txt"),
            path: String::from("fixtures/mod_scan/file1.txt"),
        }];
        assert_eq!(files, expected)
    }

    #[test]
    fn extract_todo_from_test_file() {
        let test_file = SourceFile::new(
            "file1.txt",
            "fixtures/mod_scan/file1.txt",
        );

        let expected_tag = Tag::new("bla", Some(String::from("bli")));
        let expected_todo1 = Todo::new("Test", test_file.clone(), 1, vec![]);
        let expected_todo2 = Todo::new(
            "Test",
            test_file.clone(),
            2,
            vec![expected_tag],
        );

        let todo = super::extract_todos_from_files(vec![test_file]).unwrap();

        assert_eq!(todo.len(), 2);
        assert!(todo[0].is_similar_to(&expected_todo1));
        assert!(todo[1].is_similar_to(&expected_todo2));
    }
}