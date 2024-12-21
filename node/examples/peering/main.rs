use anyhow::Result;
use grid_logger::{initialize_logger, tracing::*};
use libp2p::{ping, tcp, tls, yamux, SwarmBuilder, Transport};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    initialize_logger(2)?;

    info!("Building new network identity");

    let mut swarm = SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            tls::Config::new,
            yamux::Config::default,
        )?
        .with_behaviour(|_| ping::Behaviour::default())?
        // Allows us to observe pings indefinitely.
        .with_swarm_config(|cfg| cfg.with_idle_connection_timeout(Duration::from_secs(u64::MAX)))
        .build();

    Ok(())
}
