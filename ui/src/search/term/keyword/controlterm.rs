use crate::search::pageprinter::printer::{todoprinter::TodoPrinter, ItemPrinter};
use crate::search::term::SearchTerm;
use crate::search::term::TermDialog;
use super::super::FooterOption;
use super::searchterm::KeywordSearchTerm;

use console::Term;
use std::io::Result as IOResult;
use types::{Project, Todo};

#[derive(Clone)]
pub struct KeywordControlTerm {
    term: Term,
    items: Vec<Todo>,
    printer: TodoPrinter,
    keyword: Option<String>,
    project: Option<Project>,
    headline: String,
    quitter: Option<fn(me: Self, by_escape: bool) -> IOResult<()>>,
}

impl SearchTerm<Todo, TodoPrinter> for KeywordControlTerm {
    fn new(items: Vec<Todo>, term: Term) -> KeywordControlTerm {
        KeywordControlTerm {
            term: term,
            items: items,
            printer: TodoPrinter::new(),
            keyword: None,
            project: None,
            headline: "All Todos".to_string(),
            quitter: None,
        }
    }

    fn get_items(&self) -> &Vec<Todo> {
        &self.items
    }

    fn get_printer(&self) -> &TodoPrinter {
        &self.printer
    }

    fn get_term(&self) -> &Term {
        &self.term
    }

    fn set_on_quit(&mut self, f: fn(_: Self, _: bool) -> IOResult<()>) {
        self.quitter = Some(f);
    }

    fn char_match(&self, c: char) -> IOResult<bool> {
        match c {
            'i' => {
                self.start_keyword_search_term()?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    fn on_quit(&self, by_escape: bool) -> IOResult<()> {
        if let Some(quitter) = self.quitter {
            quitter(self.clone(), by_escape)
        } else {
            Ok(())
        }
    }

    fn get_footer_options(&self) -> Vec<FooterOption> {
        vec![FooterOption::new("i", "Start interactive search")]
    }

    fn headline(&self) -> String {
        self.headline.clone()
    }
}

impl KeywordControlTerm {
    pub fn set_headline(mut self, line: String) -> KeywordControlTerm {
        self.headline = line;
        self
    }

    pub fn set_project(mut self, project: Project) -> KeywordControlTerm {
        self.project = Some(project);
        self
    }

    pub fn set_keyword(mut self, keyword: String) -> KeywordControlTerm {
        self.keyword = Some(keyword);
        self
    }

    fn get_project(&self) -> Project {
        if let Some(project) = &self.project {
            return project.clone();
        } else {
            // Todo: Should this be the solution?
            Project::new("No Todos Found", vec![])
        }
    }

    fn get_keyword(&self) -> String {
        self.keyword.clone().unwrap_or(String::new())
    }

    fn start_keyword_search_term(&self) -> IOResult<()> {
        let mut keyword_search_term =
            KeywordSearchTerm::new(self.get_items().clone(), self.term.clone())
                .set_project(self.get_project())
                .get_filtered_search_term(&self.get_keyword());
        keyword_search_term.set_on_quit(|me: KeywordSearchTerm, by_escape: bool| {
            if by_escape {
                let term =
                    KeywordControlTerm::new(me.get_items().clone(), me.get_term().clone())
                        .set_project(me.get_project())
                        .set_keyword(me.get_keyword())
                        .set_headline(me.headline());

                let dialog = TermDialog::new(me.get_term().clone(), term);
                dialog.start()
            } else {
                Ok(())
            }
        });
        let dialog = TermDialog::new(Term::stdout(), keyword_search_term);
        dialog.start()
    }
}