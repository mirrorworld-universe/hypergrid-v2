pub mod router;
pub mod runtime;
pub mod storage;

use crate::NodeScaffolding;
use anyhow::Result;
use async_trait::async_trait;
use grid_node_core::prelude::*;
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
use router::GridRouter;
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
pub struct Grid<C: Cluster> {
    router: GridRouter<C>,
    runtime: GridRuntime<C>,
    storage: GridStorage<C>,
}

impl<C: Cluster> Grid<C> {
    pub fn new(routing_config: router::Config, runtime_config: runtime::Config) -> Result<Self> {
        let runtime = GridRuntime::<C>::new(runtime_config);

        Ok(Self {
            router: GridRouter::new(routing_config, runtime.clone()),
            // TODO: Configure GridStorage;
            storage: GridStorage::new(),
            // TODO: Configure GridRuntime;
            runtime,
        })
    }
}

//------------------------------------------
// NodeBase
//------------------------------------------

#[async_trait]
impl<C: Cluster> NodeScaffolding for Grid<C> {
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
        self.router.enable_listeners().await?;
        Ok(())
    }

    //------------------------------------------
    // Getters
    //------------------------------------------

    /// Get Node type.
    fn node_type(&self) -> NodeType {
        self.router.node_type()
    }
}
