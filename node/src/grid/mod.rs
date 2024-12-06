pub mod runtime;

use crate::{NodeInterface, NodeType};
use anyhow::Result;
use async_trait::async_trait;
use grid_node_core::Network;
use grid_node_router::Routing;
use grid_node_runtime::Runtime;
use grid_node_solana_rpc::{
    jsonrpsee::core::RpcResult,
    rpc_http::SolanaRpcServer,
    solana_rpc_client_api::config::{RpcSendTransactionConfig, RpcSimulateTransactionConfig},
};
use runtime::GridRuntime;

//------------------------------------------
// Grid
//------------------------------------------

pub struct Grid<N: Network> {
    node_type: NodeType,
    runtime: GridRuntime<N>,
}

impl<N: Network> Grid<N> {
    pub fn new(node_type: NodeType) -> Result<Self> {
        Ok(Self {
            node_type,
            // TODO: Configure GridRuntime;
            runtime: GridRuntime::<N>::new(),
        })
    }
}

impl<N: Network> Routing<N> for Grid<N> {}

#[async_trait]
impl<N: Network> SolanaRpcServer for Grid<N> {
    async fn send_transaction(
        &self,
        transaction: String,
        config: Option<RpcSendTransactionConfig>,
    ) -> RpcResult<String> {
        self.runtime.process_transaction();
        Ok(String::new())
    }

    async fn simulate_transaction(
        &self,
        transaction: String,
        config: Option<RpcSimulateTransactionConfig>,
    ) -> RpcResult<String> {
        self.runtime.process_transaction();
        Ok(String::new())
    }
}

#[async_trait]
impl<N: Network> NodeInterface<N> for Grid<N> {
    fn node_type(&self) -> NodeType {
        self.node_type
    }

    async fn shutdown(&self) {
        println!("Shutting down")
    }
}
