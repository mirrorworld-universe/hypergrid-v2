use anyhow::Result;
use async_trait::async_trait;
use grid_node_core::Network;
use grid_node_solana_rpc::{
    jsonrpsee::{server::ServerBuilder, tokio::net::ToSocketAddrs},
    rpc_http::SolanaRpcServer,
    rpc_pubsub::SolanaRpcPubSubServer,
};
use std::net::{IpAddr, SocketAddr};

/// Routing Trait.
/// Attaches Routing protocol behaviors to Node.
///
/// Important Note:
/// All Routing Protocols (Trait Bounds) attached to
/// Routing are expected to be in the same base url
/// but different ports.
#[async_trait]
// pub trait Routing<N: Network>: InboundRpcHttp + InboundRpcPubSub {
pub trait Routing<N: Network>: InboundRpcHttp {
    //------------------------------------------
    // Associated Functions
    //------------------------------------------

    /// Required method to determine which inbound
    /// Routing protocols to activate for Node.
    async fn enable_listeners(&self) -> Result<()>;

    //------------------------------------------
    // Getters
    //------------------------------------------

    /// Get base IP [`IpAddr`] of Routing
    fn ip(&self) -> IpAddr;
}

/// InboundRpcHttp Trait for Routing.
///
/// Bounds:
/// - [`Copy`] is required due to jsonrpsee proc
/// macro requirements when calling `.into_rpc()`.
///
/// - [`SolanaRpcServer`] Solana HTTP RPC standard.
///
#[async_trait]
pub trait InboundRpcHttp: Copy + SolanaRpcServer {
    //------------------------------------------
    // Associated Functions
    //------------------------------------------

    /// Enables HTTP RPC gateways.
    async fn enable_listener(&self) -> Result<()> {
        // Handle error in Node level
        let server = ServerBuilder::default().build(self.rpc_url()).await?;
        let server_handle = server.start(self.into_rpc());
        server_handle.stopped().await;
        Ok(())
    }

    /// Returns full RPC URL
    fn rpc_url(&self) -> SocketAddr;

    /// Returns Port for protocol's server
    fn port(&self) -> u16;
}

/// InboundRpcPubsub Trait for Routing.
///
/// Bounds:
/// - [`Copy`] is required due to jsonrpsee proc
/// macro requirements when calling `.into_rpc()`.
///
/// - [`SolanaRpcPubSubServer`] Solana Websocket (PubSub) RPC standard.
///
#[async_trait]
pub trait InboundRpcPubSub: Copy + SolanaRpcPubSubServer {
    //------------------------------------------
    // Associated Functions
    //------------------------------------------

    /// Enables PubSub RPC gateways.
    fn enable_listener(&self) {
        let rpc = self.into_rpc();
    }
}
