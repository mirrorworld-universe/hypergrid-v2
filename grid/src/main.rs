use clap::{crate_authors, crate_description, crate_name, crate_version, Arg, Command};
pub mod core;
use core::*;
pub mod solana;
use solana::*;

#[tokio::main]
async fn main() {
    let app_m = Command::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg_required_else_help(true)
        .subcommand(
            Command::new("start")
                .about("Grid Starter")
                .arg_required_else_help(true)
                .arg(Arg::new("MODE").long("mode").short('m').required(true)),
        )
        .get_matches();

    match app_m.subcommand() {
        Some(("start", start_m)) => {
            // let node = Node::new_sequencer(runtime, router);
            println!("{start_m:?}");
        }

        _ => {
            // Handled by clap `arg_required_else_help(true)`
            println!("unsupported command");
        }
    }
}
