use clap::{crate_authors, crate_description, crate_name, crate_version, Arg, Command};
#[tokio::main]
async fn main() {
    let app_m = Command::new(crate_name!())
        .version(crate_version!())
        .author(crate_author!())
        .about(crate_description!())
        .arg_required_else_help(true)
        .subcommand(
            Command::new("start")
                .about("Grid Starter")
                .arg_required_else_help(true)
                .arg(Arg::new("MODE").long("MODE").short('m')),
        )
        .get_matches();
}
