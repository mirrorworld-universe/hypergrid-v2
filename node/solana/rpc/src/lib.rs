pub mod rpc_http;
pub mod rpc_pubsub;

// Re-exporting to import all protocol (RPC) dependencies
// from a single source of truth (this crate)
//
// Example
// ```
//    use grid_node_solana_rpc::{
//        jsonrpsee::core::RpcResult,
//        rpc_http::SolanaRpcServer,
//        solana_rpc_client_api::config::{RpcSendTransactionConfig, RpcSimulateTransactionConfig},
//    };
// ```
pub use jsonrpsee;
pub use solana_rpc_client_api;
