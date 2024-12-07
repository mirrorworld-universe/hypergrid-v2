use grid_node_core::Network;
use grid_node_solana_rpc::{rpc_http::SolanaRpcServer, rpc_pubsub::SolanaRpcPubSubServer};

/// InboundRpcHttp Trait for Routing.
///
/// Bounds:
/// - [`Copy`] is required due to jsonrpsee proc
/// macro requirements when calling `.into_rpc()`.
///
/// - [`SolanaRpcServer`] Solana HTTP RPC standard.
///
pub trait InboundRpcHttp: Copy + SolanaRpcServer {
    /// Enables HTTP RPC gateways.
    fn enable_rpc_http(&self) {
        let rpc = self.into_rpc();
    }
}

/// InboundRpcPubsub Trait for Routing.
///
/// Bounds:
/// - [`Copy`] is required due to jsonrpsee proc
/// macro requirements when calling `.into_rpc()`.
///
/// - [`SolanaRpcPubSubServer`] Solana Websocket (PubSub) RPC standard.
///
pub trait InboundRpcPubSub: Copy + SolanaRpcPubSubServer {
    /// Enables PubSub RPC gateways.
    fn enable_rpc_pubsub(&self) {
        let rpc = self.into_rpc();
    }
}

/// Routing Trait.
///
/// Attaches Routing protocol behaviors to Node.
pub trait Routing<N: Network>: InboundRpcHttp + InboundRpcPubSub {
    /// Required method to determine which inbound
    /// Routing protocols to activate for Node.
    fn enable_listeners(&self);
}
