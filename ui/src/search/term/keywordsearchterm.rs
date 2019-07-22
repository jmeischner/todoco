use super::FooterOption;
use crate::search::pageprinter::printer::{todoprinter::TodoPrinter, ItemPrinter};
use crate::search::term::SearchTerm;
use crate::search::term::TermDialog;

use console::{style, Term};
use std::io::Result as IOResult;
use todofilter;
use types::{FilterMatch, Project, Todo};

#[derive(Clone)]
enum MatchType {
    All,
    Tags,
    Files,
    Text,
    None,
}

#[derive(Clone)]
pub struct KeywordSearchTerm {
    term: Term,
    items: Vec<Todo>,
    printer: TodoPrinter,
    keyword: Option<String>,
    project: Option<Project>,
    match_type: MatchType,
    quitter: Option<fn(term: Term) -> IOResult<()>>,
}

impl SearchTerm<Todo, TodoPrinter> for KeywordSearchTerm {
    fn new(items: Vec<Todo>, term: Term) -> KeywordSearchTerm {
        KeywordSearchTerm {
            term: term,
            items: items,
            printer: TodoPrinter::new(),
            keyword: None,
            project: None,
            match_type: MatchType::All,
            quitter: None,
        }
    }

    fn get_items(&self) -> &Vec<Todo> {
        &self.items
    }

    fn get_printer(&self) -> &TodoPrinter {
        &self.printer
    }

    fn set_on_quit(mut self, f: fn(_: Term) -> IOResult<()>) -> KeywordSearchTerm {
        self.quitter = Some(f);
        self
    }

    fn char_match(&self, c: char) -> IOResult<bool> {
        match c {
            _ => {
                let mut keyword = self.get_keyword();

                // Test for Backspace Character
                if c == '\u{7f}' && keyword.as_bytes().len() > 0 {
                    keyword.truncate(keyword.as_bytes().len() - 1);
                } else {
                    if c != '\u{7f}' {
                        keyword = format!("{}{}", keyword, c);
                    }
                }

                self.show_filtered_list(keyword)?;

                Ok(true)
            }
        }
    }

    fn on_quit(&self) -> IOResult<()> {
        if let Some(quitter) = self.quitter {
            quitter(self.term.clone())
        } else {
            Ok(())
        }
    }

    fn get_footer_options(&self) -> Vec<FooterOption> {
        vec![]
    }

    fn headline(&self) -> String {
        let found = style("Search").bold();
        let key = style(self.get_keyword()).blue();
        let by = match self.match_type {
            MatchType::Text => style("In Todo Description").cyan(),
            MatchType::Tags => style("By Tag Name").cyan(),
            MatchType::None => style("No Matching Todos Found").red(),
            MatchType::Files => style("By Match in Pathname").cyan(),
            MatchType::All => style("Match All Todos").green(),
        };

        format!("{}: {}    - {}", found, key, by)
    }
}

impl KeywordSearchTerm {
    pub fn set_project(mut self, project: Project) -> KeywordSearchTerm {
        self.project = Some(project);
        self
    }

    pub fn set_keyword(mut self, keyword: String) -> KeywordSearchTerm {
        self.keyword = Some(keyword);
        self
    }

    fn set_match_type(mut self, match_type: MatchType) -> KeywordSearchTerm {
        self.match_type = match_type;
        self
    }

    fn get_project(&self) -> Project {
        if let Some(project) = &self.project {
            return project.clone();
        } else {
            // Todo: Should this be the solution?
            Project::new("No Todos Found".to_string(), vec![])
        }
    }

    fn get_keyword(&self) -> String {
        self.keyword.clone().unwrap_or(String::new())
    }

    fn show_filtered_list(&self, keyword: String) -> IOResult<()> {
        let keyword_search_term = self
            .get_filtered_todos(&keyword)
            .set_project(self.get_project())
            .set_keyword(keyword);

        let dialog = TermDialog::new(self.term.clone(), keyword_search_term);
        dialog.start()
    }

    fn get_filtered_todos(&self, keyword: &str) -> KeywordSearchTerm {
        let term = self.term.clone();

        let filter_key = if keyword.len() > 0 {
            Some(keyword)
        } else {
            None
        };

        let filtered_todos = todofilter::get_filtered_todos(filter_key, self.get_project())
            .unwrap_or(FilterMatch::None);

        KeywordSearchTerm::new_from_filter_match(filtered_todos, term)
    }

    pub fn new_from_filter_match(filter: FilterMatch, term: Term) -> KeywordSearchTerm {
        match filter {
            FilterMatch::All(todos) => {
                KeywordSearchTerm::new(todos, term).set_match_type(MatchType::All)
            }
            FilterMatch::Files(todos) => {
                KeywordSearchTerm::new(todos, term).set_match_type(MatchType::Files)
            }
            FilterMatch::None => {
                KeywordSearchTerm::new(vec![], term).set_match_type(MatchType::None)
            }
            FilterMatch::Tags(todos) => {
                KeywordSearchTerm::new(todos, term).set_match_type(MatchType::Tags)
            }
            FilterMatch::Text(todos) => {
                KeywordSearchTerm::new(todos, term).set_match_type(MatchType::Text)
            }
        }
    }
}