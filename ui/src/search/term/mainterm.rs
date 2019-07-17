use crate::helper;
use crate::search::pageprinter::printer::itemprinter::ItemPrinter;
use crate::search::pageprinter::printer::textprinter::TextPrinter;
use crate::search::pageprinter::printer::todoprinter::TodoPrinter;
use crate::search::term::alltodosterm::AllTodosTerm;
use crate::search::term::SearchTerm;
use crate::search::term::TermDialog;
use super::FooterOption;
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
    fn new(items: Vec<String>, printer: TextPrinter, term: Term) -> MainTerm {
        MainTerm {
            term: term,
            items: items,
            printer: printer,
        }
    }

    fn get_items(&self) -> &Vec<String> {
        &self.items
    }

    fn get_printer(&self) -> &TextPrinter {
        &self.printer
    }

    fn header(&self) -> IOResult<()> {
        helper::hbar(&self.term)?;
        let welcome_line = format!("{}", style("What do you want to do?").bold());
        self.term.write_line(&welcome_line)?;
        helper::hbar(&self.term)
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

    fn get_footer_options(&self) -> Vec<FooterOption> {
        vec![
            FooterOption::new("a", "List All"),
            FooterOption::new("k", "Search by Keyword"),
            FooterOption::new("t", "List Tags")
        ]
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
        // todo: handle @error
        let project = todofilter::get_project(is_project, &current_dir).unwrap();
        let printer = TodoPrinter::new();
        let term = self.term.clone();
        let all_todos_term = AllTodosTerm::new(project.todos, printer, term.clone());
        let dialog = TermDialog::new(term, all_todos_term);
        dialog.start()
    }
}