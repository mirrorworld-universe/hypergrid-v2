use anyhow::Result;
use async_trait::async_trait;
use grid_node_solana_rpc::{
    jsonrpsee::core::RpcResult,
    rpc_http::SolanaRpcServer,
    solana_rpc_client_api::config::{RpcSendTransactionConfig, RpcSimulateTransactionConfig},
};
use std::marker::PhantomData;

pub trait Network: Send + Sync + 'static {
    const TYPE: &'static str;
}

pub trait Runtime<N: Network> {
    fn process_transaction(&self);
}

pub trait Routing<N: Network>: SolanaRpcServer {}

#[async_trait]
pub trait NodeInterface<N: Network>: Routing<N> {
    fn node_type(&self) -> NodeType;
    async fn shutdown(&self);
}

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

//------------------------------------------
// Runtime
//------------------------------------------

pub struct GridRuntime<N: Network> {
    _network: PhantomData<N>,
}

impl<N: Network> GridRuntime<N> {
    pub fn new() -> Self {
        Self {
            _network: Default::default(),
        }
    }
}

impl<N: Network> Runtime<N> for GridRuntime<N> {
    fn process_transaction(&self) {
        println!("Processing transaction...");
    }
}

//------------------------------------------
// Node
//------------------------------------------

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NodeType {
    Grid,
}

pub enum Node<N: Network> {
    Grid(Grid<N>),
}

impl<N: Network> Node<N> {
    pub fn new_grid() -> Self {
        let node_type = NodeType::Grid;
        Self::Grid(Grid::new(node_type).unwrap())
    }
}
