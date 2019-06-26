use clap::{load_yaml, App, ArgMatches};
use log::{debug, error, trace};
use std::env;
use std::path::PathBuf;


use simplelog::{Config as LogConfig, LevelFilter, TermLogger, TerminalMode};


use appconfig::AppConfig;
use export::format::taskpaper::TaskPaperBuilder;

use todoco;

// Todo: Refactor status messages to ui module
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
        println!("Start to scan project files:");
        let dir = matches.value_of("DIR").unwrap();
        let path = PathBuf::from(dir);
        // Todo: Add error handling
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
            println!("Finished scan of project and saved result.");
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
        let current_dir = todoco::list::build_current_dir_path();
        if matches.is_present("rescan") {
            println!("Rescan current directory.");
            match todoco::scan(current_dir.clone()) {
                Ok(_) => println!("Done!"),
                Err(e) => error!("{}", e),
            }
        }

        let keyword = matches.value_of("KEYWORD");
        match todoco::list(keyword, current_dir) {
            Ok(matches) => ui::print_list_matches::print(matches),
            Err(e) => error!("{}", e),
        }
    }
}

fn set_verbosity_level(matches: &ArgMatches) {
    match matches.occurrences_of("verbose") {
        1 => {
            TermLogger::init(
                LevelFilter::Debug,
                LogConfig::default(),
                TerminalMode::Mixed,
            )
            .unwrap();
            debug!("Sets verbosity level to DEBUG");
        }
        2 => {
            TermLogger::init(
                LevelFilter::Trace,
                LogConfig::default(),
                TerminalMode::Mixed,
            )
            .unwrap();
            trace!("Sets verbosity level to Trace");
        }
        0 | _ => {
            TermLogger::init(LevelFilter::Warn, LogConfig::default(), TerminalMode::Mixed).unwrap()
        }
    }
}