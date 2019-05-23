use crate::{AppConfig, List, Project, Todo};
use std::fs::File;
use std::io::Result as IOResult;
use std::io::Write;
use std::path::PathBuf;

pub struct TaskPaperBuilder<'a> {
    project: &'a Project,
}

impl<'a> TaskPaperBuilder<'a> {
    pub fn new(project: &Project) -> TaskPaperBuilder {
        TaskPaperBuilder { project: project }
    }

    pub fn build(&self) -> IOResult<Vec<u8>> {
        let mut output = Vec::new();

        writeln!(output, "{}:", self.project.name)?;

        for list in &self.project.lists {
            write_list(&list, &mut output)?;
        }

        Ok(output)
    }

    pub fn write(&self, mut path: PathBuf) -> IOResult<()> {
        let text = self.build()?;
        let extension = AppConfig::get()
            .names
            .project_directory
            .export_taskpaper_extension;
        let filename = format!("{}{}", self.project.name, extension);
        path.push(filename);
        let mut file = File::create(path)?;
        file.write(&text)?;
        Ok(())
    }
}

fn write_list(list: &List, mut writer: impl Write) -> IOResult<()> {
    writeln!(writer, "{}{}:", tab(1), list.name)?;

    for todo in &list.todos {
        write_todo(&todo, &mut writer)?;
    }

    Ok(())
}

fn write_todo(todo: &Todo, mut writer: impl Write) -> IOResult<()> {
    write!(writer, "{}- {}", tab(2), todo.text)?;
    for tag in &todo.tags {
        if let Some(value) = &tag.value {
            write!(writer, " @{}({})", tag.name, value)?;
        } else {
            write!(writer, " @{}", tag.name)?;
        }
    }
    writeln!(writer, "")?;
    writeln!(writer, "{}in {}:{}", tab(2), todo.file.path, todo.line)?;

    Ok(())
}

fn tab(times: usize) -> String {
    "    ".repeat(times)
}

// ~~~~~~~~~~~~~~~~~~~~ TESTS ~~~~~~~~~~~~~~~~~~~~ //
#[cfg(test)]
mod tests_taskpaper_parser {
    use super::TaskPaperBuilder;
    use crate::{List, Project, SourceFile, Tag, Todo};

    #[test]
    fn create_taskpaper_string_from_project() {
        let project = Project::new(
            String::from("Test"),
            vec![List::new(
                String::from("List 1"),
                vec![Todo::new(
                    String::from("Todo 1"),
                    SourceFile::new(String::from("p.txt"), String::from("path/p.txt")),
                    34,
                    vec![
                        Tag::new(String::from("bla"), None),
                        Tag::new(String::from("bli"), Some(String::from("blubb"))),
                    ],
                )],
            )],
        );
        let result_text = String::from_utf8(TaskPaperBuilder::new(&project).build().unwrap());
        let expected = "Test:
    List 1:
        - Todo 1 @bla @bli(blubb)
        in path/p.txt:34
";
        assert_eq!(expected, result_text.unwrap());
    }
}
