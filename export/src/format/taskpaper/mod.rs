use appconfig::AppConfig;
use std::fs::File;
use std::io::Result as IOResult;
use std::io::Write;
use std::path::PathBuf;
use types::{Project, Todo};

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

        for todo in &self.project.todos {
            write_todo(&todo, &mut output)?;
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
    use types::{Project, SourceFile, Tag, Todo};

    #[test]
    fn create_taskpaper_string_from_project() {
        let project = Project::new(
            String::from("Test"),
            vec![Todo::new(
                String::from("Todo 1"),
                SourceFile::new(String::from("p.txt"), String::from("path/p.txt")),
                34,
                vec![
                    Tag::new(String::from("bla"), None),
                    Tag::new(String::from("bli"), Some(String::from("blubb"))),
                ],
            )],
        );
        let result_text = String::from_utf8(TaskPaperBuilder::new(&project).build().unwrap());
        let expected = "Test:
        - Todo 1 @bla @bli(blubb)
        in path/p.txt:34
";
        assert_eq!(expected, result_text.unwrap());
    }
}
