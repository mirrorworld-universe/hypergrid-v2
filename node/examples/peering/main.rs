use anyhow::Result;
use grid_logger::{initialize_logger, tracing::*};
use libp2p::{tcp, tls, yamux, SwarmBuilder, Transport};

#[tokio::main]
async fn main() -> Result<()> {
    initialize_logger(2)?;

    info!("Building new network identity");

    let mut swarm = SwarmBuilder::with_new_identity().with_tokio().with_tcp(
        tcp::Config::default(),
        tls::Config::new,
        yamux::Config::default,
    )?;

    Ok(())
}
