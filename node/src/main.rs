mod cli;
mod config;
mod node;

use anyhow::Result;
use clap::{ArgMatches, Command};
use cli::CliHandler;
use handlers::StartHandler;

#[tokio::main]
async fn main() -> Result<()> {
    let toplevel_matches = Command::new("grid")
        // .version("0.1.0") // commented out to get version from manifest
        .author("Sonic Engineering <engineering@sonic.game>")
        .about("Grid V2")
        .arg_required_else_help(true)
        .subcommand(Command::new("start").about("Run the Grid node"))
        .get_matches();

    match toplevel_matches.subcommand() {
        Some(("start", sub_matches)) => StartHandler::handle_matches(sub_matches).await?,

        _ => {
            // clap handles this case with
        }
    }

    Ok(())
}

// Temporarily merging all handlers under main
mod handlers {
    use super::*;
    use async_trait::async_trait;
    use grid_mock_storage::MockStorage;
    use grid_solana_svm::runtime::SvmRuntime;
    use node::GridNode;

    pub struct StartHandler;

    #[async_trait]
    impl CliHandler for StartHandler {
        async fn handle_matches(matches: &ArgMatches) -> Result<()> {
            let runtime = SvmRuntime::new();
            let storage = MockStorage::new();
            let node = GridNode::new(runtime, storage);

            // move node into task
            tokio::spawn(async move { node.start().await }).await??;

            Ok(())
        }
    }
}
