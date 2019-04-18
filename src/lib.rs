mod scan;

pub use scan::data_structures::Project;
pub use scan::data_structures::List;
pub use scan::data_structures::Todo;

pub fn scan(root_dir: String) -> Project {
    Project {
        name: String::from("ToDoco"),
        lists: vec!(),
    }
}