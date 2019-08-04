use super::FooterOption;
use crate::helper;
use crate::search;
use crate::search::pageprinter::printer::{textprinter::TextPrinter, ItemPrinter};
use crate::search::term::{AllTagsTerm, AllTodosTerm, KeywordControlTerm, SearchTerm, TermDialog};
use console::{style, Term};
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

    fn get_term(&self) -> &Term {
        &self.term
    }

    fn set_on_quit(&mut self, _: fn(_: Self, _: bool) -> IOResult<()>) {}

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
            't' => {
                self.all_tags_dialog()?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    fn on_quit(&self, _: bool) -> IOResult<()> {
        self.term.clear_screen()?;

        let goodbye_line = helper::get_goodbye_message();

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
        let mut keyword_control_term =
            KeywordControlTerm::new(project.todos.clone(), self.term.clone()).set_project(project);
        keyword_control_term.set_on_quit(|_, _| search::start());
        let dialog = TermDialog::new(self.term.clone(), keyword_control_term);
        dialog.start()
    }

    fn all_tags_dialog(&self) -> IOResult<()> {
        let current_dir = todofilter::build_current_dir_path();
        let (is_project, _config) = types_helper::get_config_and_project_info_from(&current_dir);
        // todo: handle @error
        let project = todofilter::get_project(is_project, &current_dir).unwrap();
        let term = self.term.clone();
        let tags = project
            .get_tags()
            .iter()
            .cloned()
            .map(|tag| tag.clone())
            .collect();
        let tags_term = AllTagsTerm::new(tags, term.clone());
        let dialog = TermDialog::new(term, tags_term);
        dialog.start()
    }
}