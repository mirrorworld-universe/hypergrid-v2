mod cli;
mod node;

use anyhow::Result;
use clap::{ArgMatches, Command};
use cli::CliHandler;

fn main() -> Result<()> {
    let toplevel_matches = Command::new("grid")
        // .version("0.1.0") // commented out to get version from manifest
        .author("Sonic Engineering <engineering@sonic.game>")
        .about("Grid V2")
        .arg_required_else_help(true)
        .subcommand(Command::new("start").about("Run the Grid node"))
        .get_matches();

    match toplevel_matches.subcommand() {
        Some(("start", sub_matches)) => StartHandler::handle_matches(sub_matches)?,

        _ => {
            // clap handles this case with
        }
    }

    Ok(())
}

pub struct StartHandler;

impl CliHandler for StartHandler {
    fn handle_matches(matches: &ArgMatches) -> Result<()> {
        Ok(())
    }
}
