
use log::debug;
use std::cmp;
/// Struct to handle paging of
/// a vector of items
///
/// # Properties
/// `items` - holds the full vector elements to page
/// `height` - number of rows in output
#[derive(Clone)]
pub struct Pager<'a, I> {
    items: &'a Vec<I>,
    height: usize,
    current_page: usize,
}

impl<'a, I> Pager<'a, I> {

    /// Constructor
    pub fn new(items: &'a Vec<I>, height: usize) -> Pager<I> {
        Pager {
            items: items,
            height: height,
            current_page: 1,
        }
    }

    /// Get the elements of the current page
    pub fn current(&self) -> &[I] {
        self.get_page(self.current_page)
    }

    /// Get the elements of the next page
    pub fn next(&mut self) -> &[I] {
        if self.current_page == self.get_max_page_number() {
            &self.current()
        } else {
            self.current_page += 1;
            &self.current()
        }
    }

    /// Get the elements of the previous page
    pub fn prev(&mut self) -> &[I] {
        if self.current_page == 1 {
            &self.current()
        } else {
            self.current_page -= 1;
            &self.current()
        }
    }

    /// Returns the number of lines on one page
    pub fn page_height(&self) -> usize {
        self.height
    }

    /// Function to get items of given page
    ///
    /// # Arguments
    /// `page` - one based page number
    fn get_page(&self, page: usize) -> &[I] {
        let start_index = (page - 1) * self.get_items_per_page();
        let end_index = cmp::min(page * self.get_items_per_page(), self.items.len());
        // debug!("Start: {}, End: {}", start_index, end_index);
        &self.items[start_index..end_index]
    }

    /// Returns the maximum number of pages
    fn get_max_page_number(&self) -> usize {
        let result = self.items.len() as f32 / self.get_items_per_page() as f32;
        result.ceil() as usize
    }

    /// Returns the number of list items
    /// per page
    fn get_items_per_page(&self) -> usize {
        self.height
    }
}

// ~~~~~~~~~~~~~~~~~~~~ TESTS ~~~~~~~~~~~~~~~~~~~~ //
#[cfg(test)]
mod tests_pager {
    use super::Pager;
    use types::{SourceFile, Todo};
    #[test]
    fn test_get_page() {

        let file = SourceFile::new("my file.txt".to_string(), "here/it/is".to_string());
        let todo_1 = Todo::new("Todo 1".to_string(), file.clone(), 1, vec![]);
        let todo_2 = Todo::new("Todo 2".to_string(), file.clone(), 6, vec![]);
        let todo_3 = Todo::new("Todo 3".to_string(), file.clone(), 15, vec![]);
        let todo_4 = Todo::new("Todo 4".to_string(), file.clone(), 22, vec![]);
        let todo_5 = Todo::new("Todo 5".to_string(), file.clone(), 56, vec![]);
        let todo_6 = Todo::new("Todo 6".to_string(), file.clone(), 69, vec![]);
        let todos = vec![todo_1, todo_2, todo_3, todo_4, todo_5, todo_6];

        let mut pager = Pager::new(&todos, 4);

        let page_0 = pager.current();
        assert_eq!(page_0, &todos[0..4]);

        let page_1 = pager.prev();
        assert_eq!(page_1, &todos[0..4]);

        let page_2 = pager.next();
        assert_eq!(page_2, &todos[4..]);

        let page_3 = pager.next();
        assert_eq!(page_3, &todos[4..]);

        let page_4 = pager.next();
        assert_eq!(page_4, &todos[4..]);

        let page_5 = pager.prev();
        assert_eq!(page_5, &todos[0..4]);
    }

    #[test]
    fn test_max_page_number() {
        let file = SourceFile::new("my file.txt".to_string(), "here/it/is".to_string());
        let todo_1 = Todo::new("Todo 1".to_string(), file.clone(), 1, vec![]);
        let todo_2 = Todo::new("Todo 2".to_string(), file.clone(), 6, vec![]);
        let todo_3 = Todo::new("Todo 3".to_string(), file.clone(), 15, vec![]);
        let todo_4 = Todo::new("Todo 4".to_string(), file.clone(), 22, vec![]);
        let todo_5 = Todo::new("Todo 5".to_string(), file.clone(), 56, vec![]);
        let todos = vec![todo_1, todo_2, todo_3, todo_4, todo_5];

        let pager_1 = Pager::new(&todos, 3);
        let pager_2 = Pager::new(&todos, 2);

        assert_eq!(pager_1.get_max_page_number(), 2);
        assert_eq!(pager_2.get_max_page_number(), 3);
    }
}