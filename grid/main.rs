pub mod core;
pub mod solana;

use anyhow::Result;
use clap::{crate_authors, crate_description, crate_name, crate_version, Arg, Command, ValueEnum};
use core::*;
use solana::*;

#[derive(Clone, Debug, Eq, PartialEq, ValueEnum)]
pub enum NodeMode {
    Grid,
}

#[tokio::main]
async fn main() -> Result<()> {
    let app_m = Command::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg_required_else_help(true)
        .subcommand(
            Command::new("start")
                .about("Grid Starter")
                .arg_required_else_help(true)
                .arg(
                    Arg::new("MODE")
                        .long("mode")
                        .short('m')
                        .value_parser(clap::value_parser!(NodeMode))
                        .required(true),
                )
                .arg(
                    Arg::new("RPC_HTTP_URL")
                        .long("rpc-http")
                        .default_value("127.0.0.1"),
                )
                .arg(
                    Arg::new("RPC_HTTP_PORT")
                        .long("rpc-http-port")
                        .value_parser(clap::value_parser!(u16))
                        .default_value("8080"),
                ),
        )
        .get_matches();

    match app_m.subcommand() {
        Some(("start", start_m)) => {
            let router_config = SolanaSvmRoutingConfig::new("127.0.0.1", 8080);
            if let Node::Grid(node) = Node::new_grid(router_config) {
                node.start().await?;
            }
        }

        _ => {
            // Handled by clap `arg_required_else_help(true)`
            println!("unsupported command");
        }
    }

    Ok(())
}
