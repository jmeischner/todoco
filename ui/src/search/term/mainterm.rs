use crate::helper;
use crate::search::pageprinter::printer::itemprinter::ItemPrinter;
use crate::search::pageprinter::printer::todoprinter::TodoPrinter;
use crate::search::pageprinter::printer::textprinter::TextPrinter;
use crate::search::term::alltodosterm::AllTodosTerm;
use crate::search::term::SearchTerm;
use crate::search::term::TermDialog;
use console::{style, Term};
use std::io::Result as IOResult;
use todofilter;
use types::config::helper as types_helper;

#[derive(Clone)]
pub struct MainTerm {
    term: Option<Term>,
    items: Vec<String>,
    printer: TextPrinter,
}

impl SearchTerm<String, TextPrinter> for MainTerm {
    fn new(items: Vec<String>, printer: TextPrinter) -> MainTerm {
        MainTerm {
            term: None,
            items: items,
            printer: printer,
        }
    }

    fn set_term(mut self, term: Term) -> MainTerm {
        self.term = Some(term);
        self
    }

    fn get_term(&self) -> Term {
        if let Some(term) = &self.term {
            term.clone()
        } else {
            Term::stdout()
        }
    }

    fn get_items(&self) -> &Vec<String> {
        &self.items
    }

    fn get_printer(&self) -> &TextPrinter {
        &self.printer
    }

    fn header(&self) -> IOResult<()> {
        helper::hbar(&self.get_term())?;
        let welcome_line = format!("{}", style("What do you want to do?").bold());
        self.get_term().write_line(&welcome_line)?;
        helper::hbar(&self.get_term())
    }


    fn char_match(&self, c: char) -> IOResult<bool> {
        match c {
            'q' => Ok(true),
            'a' => {
                self.all_todos_dialog()?;
                return Ok(true);
            }
            _ => Ok(false),
        }
    }

    fn on_loop_end(&self) -> IOResult<()> {
        Ok(())
    }

    fn footer(&self) -> IOResult<()> {
        helper::hbar(&self.get_term())?;
        let menu_line = format!(
            "{}: Next Page | {}: Previous Page | {}: List All | {}: Search by Keyword | {}: List Tags | {}: Quit",
            style("▶|▼").blue(),
            style("◀|▲").blue(),
            style("a").blue(),
            style("k").blue(),
            style("t").blue(),
            style("q").blue(),
        );
        self.get_term().write_line(&menu_line)
    }

    fn get_header_lines(&self) -> usize {
        3
    }

    fn get_footer_lines(&self) -> usize {
        2
    }
}

impl MainTerm {
    fn all_todos_dialog(&self) -> IOResult<()> {
        let current_dir = todofilter::build_current_dir_path();
        let (is_project, _config) = types_helper::get_config_and_project_info_from(&current_dir);
        // todo: handle error @error
        let project = todofilter::get_project(is_project, &current_dir).unwrap();
        let printer = TodoPrinter::new();
        let all_todos_term = AllTodosTerm::new(project.todos, printer);
        let term = self.get_term();
        let dialog = TermDialog::new(term, all_todos_term);
        dialog.start()
    }
}