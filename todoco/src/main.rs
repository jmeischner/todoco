use clap::{load_yaml, App, ArgMatches};
use log::{debug, error, info, trace, warn};
use std::env;
use std::path::PathBuf;


use simplelog::{Config as LogConfig, LevelFilter, TermLogger, TerminalMode};


use appconfig::AppConfig;
use export::format::taskpaper::TaskPaperBuilder;

use todoco;

fn main() {

    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml)
        .version(clap::crate_version!())
        .get_matches();

    set_verbosity_level(&matches);
    handle_scan(&matches);
    handle_init(&matches);
    handle_list(&matches);
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
                error!("{}", e);
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
                error!("{}", e);
            } else {
                println!("wrote project file")
            }
        } else {
            error!("Could not detect current directory.")
        }
    }
}

fn handle_list(matches: &ArgMatches) {
    if let Some(matches) = matches.subcommand_matches("list") {
        let keyword = matches.value_of("KEYWORD");
        match todoco::list(keyword) {
            Ok(matches) => ui::print_list_matches::print(matches),
            Err(e) => error!("{}", e),
        }
    }
}

fn set_verbosity_level(matches: &ArgMatches) {
    match matches.occurrences_of("verbose") {
        1 => {
            TermLogger::init(LevelFilter::Warn, LogConfig::default(), TerminalMode::Mixed).unwrap();
            warn!("Sets verbosity level to WARN");
        }
        2 => {
            TermLogger::init(LevelFilter::Info, LogConfig::default(), TerminalMode::Mixed).unwrap();
            info!("Sets verbosity level to INFO");
        }
        3 => {
            TermLogger::init(
                LevelFilter::Debug,
                LogConfig::default(),
                TerminalMode::Mixed,
            )
            .unwrap();
            debug!("Sets verbosity level to DEBUG");
        }
        4 => {
            TermLogger::init(
                LevelFilter::Trace,
                LogConfig::default(),
                TerminalMode::Mixed,
            )
            .unwrap();
            trace!("Sets verbosity level to TRACE");
        }
        0 | _ => TermLogger::init(
            LevelFilter::Error,
            LogConfig::default(),
            TerminalMode::Mixed,
        )
        .unwrap(),
    }
}