use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("ToDoCo")
        .version(clap::crate_version!())
        .author("Jan Meischner <jan.meischner@googlemail.com>")
        .about("Extracts Todos from Code")
        .get_matches();

    todoco::scan("**/*.*");
}
