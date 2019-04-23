mod scan;

pub use scan::sourcefile::SourceFile;
pub use scan::list::List;
pub use scan::project::Project;
pub use scan::todo::Todo;

pub fn scan(root_dir: String) -> Project {
    Project {
        name: String::from("ToDoco"),
        lists: vec![],
    }
}
