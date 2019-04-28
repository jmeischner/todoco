use clap::{App, Arg, SubCommand};
use todoco;

mod ui;

fn main() {
    // Todo: create *cli.yml* -- see docs
    let matches = App::new("ToDoCo")
        .version(clap::crate_version!())
        .author("Jan Meischner <jan.meischner@googlemail.com>")
        .about("Extracts Todos from Code")
        .subcommand(
            SubCommand::with_name("scan")
                .about("Scan directory for ToDo comments")
                .arg(
                    Arg::with_name("DIR")
                        .help("Shell glob for files which should get scanned.")
                        .required(true),
                ),
        )
        .subcommand(SubCommand::with_name("init").about("Initialize new ToDoCo project"))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("scan") {
        let dir = matches.value_of("DIR").unwrap();
        let project = todoco::scan(dir);
        ui::print_todo_list::print_project(project);
    }

    if let Some(_matches) = matches.subcommand_matches("init") {
        let config = ui::dialog_config::ask_for_config().unwrap();
        if let Err(e) = todoco::init(config) {
            eprintln!("{}", e);
        }
    }
}
