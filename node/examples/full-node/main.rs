use anyhow::Result;
use futures::prelude::*;
use grid_logger::{initialize_logger, tracing::*};
use libp2p::{ping, swarm::SwarmEvent, tcp, tls, yamux, Multiaddr, SwarmBuilder, Transport};
use std::time::Duration;

pub struct Node;

impl Node {
    pub fn new() -> Self {
        Self {}
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    initialize_logger(2)?;

    let node = Node::new();

    Ok(())
}
