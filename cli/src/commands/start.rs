use anyhow::Result;
use clap::Parser;

use std::net::SocketAddr;

#[derive(Clone, Debug, Parser)]
pub struct Start {
    /// Specify the IP address and port for the node server
    #[clap(long = "node")]
    pub node: Option<SocketAddr>,
}

impl Start {
    /// Starts the Grid node
    pub fn parse(self) -> Result<String> {
        Ok(String::new())
    }
}
