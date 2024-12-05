use anyhow::Result;
use async_trait::async_trait;
use clap::ArgMatches;

#[async_trait]
pub trait CliHandler {
    async fn handle_matches(matches: &ArgMatches) -> Result<()>;
}
