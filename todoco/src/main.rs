use clap::{load_yaml, App, ArgMatches};
use log::{debug, error, trace};
use simplelog::{Config as LogConfig, LevelFilter, TermLogger, TerminalMode};
use std::env;
use std::path::{Path, PathBuf};

use appconfig::AppConfig;
use export::format::taskpaper::TaskPaperBuilder;
use todoco;
use todofilter;

use ui::{config_dialog, search};

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
    handle_search(&matches);
}

fn handle_scan(matches: &ArgMatches) {
    if let Some(matches) = matches.subcommand_matches("scan") {

        let dir = matches.value_of("DIR").unwrap();
        let path = PathBuf::from(dir);

        if !path_is_project(&path) {
            println!("Directory is no ToDoCo project.");
            println!("Use `todoco init` to initialize a new project.");
            println!("Or use `todoco list` to list all ToDos in current directory.");
            return;
        };

        println!("Start to scan project files:");
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
            let config = config_dialog::ask_for_config().unwrap();
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
        let current_dir = todofilter::build_current_dir_path();
        if matches.is_present("rescan") {
            println!("Rescan current directory.");
            match todoco::scan(current_dir.clone()) {
                Ok(_) => println!("Done!"),
                Err(e) => error!("{}", e),
            }
        }

        let keyword = matches.value_of("KEYWORD");
        // Todo: Use ref instead of cloning PathBuf
        match todofilter::get_filtered_todos_by_path(keyword, current_dir.clone()) {
            Ok(matches) => {
                // Todo: Handle IOResult
                search::list(keyword, matches, current_dir).unwrap();
            }
            Err(e) => error!("{}", e),
        }
    }
}

fn handle_search(matches: &ArgMatches) {
    if let Some(_matches) = matches.subcommand_matches("search") {
        match search::start() {
            Ok(_) => {}
            Err(e) => error!("Something went wrong: {}", e),
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

fn path_is_project(path: &Path) -> bool {
    let project_file = appconfig::AppConfig::get().names.project_file;
    let mut path = path.to_path_buf();
    path.push(project_file);
    path.exists()
}