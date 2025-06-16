pub mod config;

use anyhow::Result;
use clap::{crate_authors, crate_description, crate_name, crate_version, Arg, Command, ValueEnum};
use core::*;
use solana::*;

#[tokio::main]
async fn main() -> Result<()> {
    let app_m = Command::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg_required_else_help(true)
        .subcommand(
            Command::new("grid")
                .about("Hypergrid Grid")
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
                        .default_value("0.0.0.0"),
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
        Some(("grid", start_m)) => {
            let rpc_http_url = start_m
                .get_one::<String>("RPC_HTTP_URL")
                .expect("RPC_HTTP_URL is defaulted");
            let rpc_http_port = start_m
                .get_one::<u16>("RPC_HTTP_PORT")
                .expect("RPC_HTTP_PORT is defaulted");
            let router_config = SolanaSvmRoutingConfig::new(rpc_http_url, *rpc_http_port);

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
