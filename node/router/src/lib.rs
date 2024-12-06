use grid_node_core::Network;
use grid_node_solana_rpc::rpc_http::SolanaRpcServer;

pub trait Routing<N: Network>: SolanaRpcServer {}
