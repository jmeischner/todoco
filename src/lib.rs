pub mod scan;

pub use scan::list::List;
pub use scan::project::Project;
pub use scan::sourcefile::SourceFile;
pub use scan::todo::Todo;

pub fn scan(root_dir: &str) -> Project {
    let files = scan::get_files(root_dir);
    let todos = scan::extract_todos_from_files(files.unwrap());

    for todo in todos {
        println!("{:?}", todo);
    }

    Project {
        name: String::from("ToDoco"),
        lists: vec![],
    }
}
