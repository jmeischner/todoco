#[macro_use]
extern crate clap;

use clap::{App, ArgMatches};
use std::env;
use std::path::PathBuf;

use export::format::taskpaper::TaskPaperBuilder;
use appconfig::AppConfig;
use ui;

use todoco;

fn main() {
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml)
        .version(clap::crate_version!())
        .get_matches();

    handle_scan(&matches);
    handle_init(&matches);
}

fn handle_scan(matches: &ArgMatches) {
    if let Some(matches) = matches.subcommand_matches("scan") {
        let dir = matches.value_of("DIR").unwrap();
        let path = PathBuf::from(dir);
        let project = todoco::scan(path.clone()).unwrap();

        // Todo: Build enum of export options @prio(1) @next-week
        if matches.is_present("export_taskpaper") {
            let builder = TaskPaperBuilder::new(&project);
            if let Err(e) = builder.write(path.clone()) {
                eprintln!("{}", e);
            } else {
                let extension = AppConfig::get()
                    .names
                    .project_directory
                    .export_taskpaper_extension;
                println!("Finished! You got your {}{}!", project.name, extension);
            };
        } else {
            ui::print_todo_list::print_project(&project);
        }
    }
}

fn handle_init(matches: &ArgMatches) {
    if let Some(_matches) = matches.subcommand_matches("init") {
        if let Ok(cur_dir) = env::current_dir() {
            let config = ui::dialog_config::ask_for_config().unwrap();
            if let Err(e) = todoco::init(config, cur_dir) {
                eprintln!("{}", e);
            } else {
                println!("wrote project file")
            }
        } else {
            eprintln!("Could not detect current directory.")
        }
    }
}
