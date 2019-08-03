use super::super::FooterOption;
use super::searchterm::KeywordSearchTerm;
use super::MatchType;
use crate::search::pageprinter::printer::{todoprinter::TodoPrinter, ItemPrinter};
use crate::search::term::SearchTerm;
use crate::search::term::TermDialog;

use console::{style, Term};
use std::io::Result as IOResult;
use types::{Project, Todo};

#[derive(Clone)]
pub struct KeywordControlTerm {
    term: Term,
    items: Vec<Todo>,
    printer: TodoPrinter,
    keyword: Option<String>,
    project: Option<Project>,
    match_type: MatchType,
    quitter: Option<fn(me: Self) -> IOResult<()>>,
}

impl SearchTerm<Todo, TodoPrinter> for KeywordControlTerm {
    fn new(items: Vec<Todo>, term: Term) -> KeywordControlTerm {
        KeywordControlTerm {
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

    fn get_term(&self) -> &Term {
        &self.term
    }

    fn set_on_quit(mut self, f: fn(_: Self) -> IOResult<()>) -> KeywordControlTerm {
        self.quitter = Some(f);
        self
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

    fn on_quit(&self) -> IOResult<()> {
        if let Some(quitter) = self.quitter {
            quitter(self.clone())
        } else {
            Ok(())
        }
    }

    fn get_footer_options(&self) -> Vec<FooterOption> {
        vec![FooterOption::new("i", "Start interactive search")]
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

impl KeywordControlTerm {
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
        let keyword_search_term =
            KeywordSearchTerm::new(self.get_project().get_todos().clone(), self.term.clone())
                .get_filtered_todos(&self.get_keyword())
                .set_keyword(self.get_keyword())
                .set_project(self.get_project())
                .set_on_quit(|me: KeywordSearchTerm| {
                    let term =
                        KeywordControlTerm::new(me.get_items().clone(), me.get_term().clone())
                            .set_project(me.get_project())
                            .set_keyword(me.get_keyword());
                    let dialog = TermDialog::new(me.get_term().clone(), term);
                    dialog.start()
                });
        let dialog = TermDialog::new(Term::stdout(), keyword_search_term);
        dialog.start()
    }
}