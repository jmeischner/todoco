use clap::{App, Arg, SubCommand};

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
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("scan") {
        let dir = matches.value_of("DIR").unwrap();
        let project = todoco::scan(dir);
        println!("{:?}", project);
    }
}
