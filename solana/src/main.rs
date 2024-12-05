use clap::Parser;
use grid_cli::commands::Cli;
use std::process::exit;

fn main() -> anyhow::Result<()> {
    // Parse the given arguments.
    let cli = Cli::parse();

    // Run the CLI.
    match cli.command.parse() {
        Ok(output) => println!("{output}\n"),
        Err(error) => {
            println!("⚠️  {error}\n");
            exit(1);
        }
    }
    Ok(())
}
