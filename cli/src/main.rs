use anyhow::Result;
use clap::{Arg, ArgAction, Command};

fn main() -> Result<()> {
    let matches = Command::new("grid")
        // .version("0.1.0") // commented out to get version from manifest
        .about("Grid V2")
        .arg_required_else_help(true)
        .arg(
            Arg::new("TEST")
                .help("Test flag")
                .long("test")
                .short('t')
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    println!("{}", matches.get_flag("TEST"));

    Ok(())
}
