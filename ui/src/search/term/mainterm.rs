use super::FooterOption;
use crate::search;
use crate::search::pageprinter::printer::{textprinter::TextPrinter, ItemPrinter};
use crate::search::term::SearchTerm;
use crate::search::term::TermDialog;
use crate::search::term::{AllTodosTerm, KeywordSearchTerm};
use console::{style, Emoji, Term};
use std::io::Result as IOResult;
use todofilter;
use types::config::helper as types_helper;

#[derive(Clone)]
pub struct MainTerm {
    term: Term,
    items: Vec<String>,
    printer: TextPrinter,
}

impl SearchTerm<String, TextPrinter> for MainTerm {
    fn new(items: Vec<String>, term: Term) -> MainTerm {
        MainTerm {
            term: term,
            items: items,
            printer: TextPrinter::new(),
        }
    }

    fn get_items(&self) -> &Vec<String> {
        &self.items
    }

    fn set_on_quit(self, _: fn() -> IOResult<()>) -> MainTerm {
        self
    }

    fn get_printer(&self) -> &TextPrinter {
        &self.printer
    }

    fn headline(&self) -> String {
        format!("{}", style("What do you want to do?").bold())
    }


    fn char_match(&self, c: char) -> IOResult<bool> {
        match c {
            'a' => {
                self.all_todos_dialog()?;
                Ok(true)
            }
            'k' => {
                self.search_by_keyword()?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    fn on_quit(&self) -> IOResult<()> {
        self.term.clear_screen()?;

        let goodbye_line = format!(
            "Goodbye {} and Thank You {}",
            Emoji("👋", ""),
            Emoji("🙏", "")
        );

        self.term
            .write_line(&format!("{}", style(goodbye_line).bold()))
    }

    fn get_footer_options(&self) -> Vec<FooterOption> {
        vec![
            FooterOption::new("a", "List All"),
            FooterOption::new("k", "Search by Keyword"),
            FooterOption::new("t", "List Tags"),
        ]
    }
}

impl MainTerm {
    fn all_todos_dialog(&self) -> IOResult<()> {
        let current_dir = todofilter::build_current_dir_path();
        let (is_project, _config) = types_helper::get_config_and_project_info_from(&current_dir);
        // todo: handle @error
        let project = todofilter::get_project(is_project, &current_dir).unwrap();
        let term = self.term.clone();
        let all_todos_term = AllTodosTerm::new(project.todos, term.clone());
        let dialog = TermDialog::new(term, all_todos_term);
        dialog.start()
    }

    fn search_by_keyword(&self) -> IOResult<()> {
        let current_dir = todofilter::build_current_dir_path();
        let (is_project, _config) = types_helper::get_config_and_project_info_from(&current_dir);
        // todo: handle @error
        let project = todofilter::get_project(is_project, &current_dir).unwrap();
        let term = self.term.clone();
        let keyword_search_term = KeywordSearchTerm::new(project.todos.clone(), term.clone())
            .set_project(project)
            .set_on_quit(search::start);
        let dialog = TermDialog::new(term, keyword_search_term);
        dialog.start()
    }
}