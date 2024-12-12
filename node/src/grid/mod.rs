pub mod runtime;
pub mod storage;

use crate::{NodeScaffolding, NodeType};
use anyhow::Result;
use async_trait::async_trait;
use grid_node_core::Network;
use grid_node_router::{InboundRpcHttp, InboundRpcPubSub, Routing};
use grid_node_runtime::Runtime;
use grid_node_solana_rpc::{
    jsonrpsee::{
        core::{RpcResult, SubscriptionResult},
        PendingSubscriptionSink,
    },
    rpc_http::SolanaRpcServer,
    rpc_pubsub::SolanaRpcPubSubServer,
    solana_rpc_client_api::config::{RpcSendTransactionConfig, RpcSimulateTransactionConfig},
};
use runtime::GridRuntime;
use std::{
    future::Future,
    net::{IpAddr, SocketAddr},
};
use storage::GridStorage;

//------------------------------------------
// Grid
//------------------------------------------

/// Grid.
///
/// A specific type of Node.
///
/// The main Node implementation critical to
/// bootstrapping the network.
///
#[derive(Clone, Debug)]
pub struct Grid<N: Network> {
    node_type: NodeType,
    node_ip: IpAddr,
    rpc_port: u16,
    rpc_pubsub_port: u16,
    runtime: GridRuntime<N>,
    storage: GridStorage<N>,
}

impl<N: Network> Grid<N> {
    pub fn new(
        node_ip: IpAddr,
        rpc_port: u16,
        rpc_pubsub_port: u16,
        node_type: NodeType,
    ) -> Result<Self> {
        Ok(Self {
            node_ip,
            rpc_port,
            rpc_pubsub_port,
            node_type,
            // TODO: Configure GridStorage;
            storage: GridStorage::<N>::new(),
            // TODO: Configure GridRuntime;
            runtime: GridRuntime::<N>::new(),
        })
    }
}

//------------------------------------------
// NodeBase
//------------------------------------------

#[async_trait]
impl<N: Network> NodeScaffolding<N> for Grid<N> {
    //------------------------------------------
    // Associated Functions
    //------------------------------------------

    fn prepare(&self) {}

    fn shutdown(&self) {}

    //------------------------------------------
    // Asynchronous Associated Functions
    //------------------------------------------

    /// Runs Node and initial services.
    async fn run(&self) -> Result<()> {
        self.enable_listeners().await?;
        Ok(())
    }

    //------------------------------------------
    // Getters
    //------------------------------------------

    /// Get Node type.
    fn node_type(&self) -> NodeType {
        self.node_type
    }
}

//------------------------------------------
// Routing
//------------------------------------------

// Routing
#[async_trait]
impl<N: Network> Routing<N> for Grid<N> {
    /// Enable all Routing listeners
    async fn enable_listeners(&self) -> Result<()> {
        self.enable_listener().await?;
        Ok(())
    }

    fn ip(&self) -> IpAddr {
        self.node_ip
    }
}

// InboundRpcHttp
#[async_trait]
impl<N: Network> InboundRpcHttp for Grid<N> {
    fn rpc_url(&self) -> SocketAddr {
        SocketAddr::new(self.ip(), self.port())
    }

    fn port(&self) -> u16 {
        self.rpc_port
    }
}

// SolanaRpcServer
#[async_trait]
impl<N: Network> SolanaRpcServer for Grid<N> {
    async fn send_transaction(
        &self,
        transaction: String,
        config: Option<RpcSendTransactionConfig>,
    ) -> RpcResult<String> {
        self.runtime.process_transaction();
        println!("Transaction: {:?}", transaction);
        println!("Config: {:?}", config);
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

// // InboundRpcPubSub
// impl<N: Network> InboundRpcPubSub for Grid<N> {}
//
// // SolanaRpcPubSubServer
// #[async_trait]
// impl<N: Network> SolanaRpcPubSubServer for Grid<N> {
//     async fn slot_subscribe(&self, pending: PendingSubscriptionSink) -> SubscriptionResult {
//         Ok(())
//     }
// }
