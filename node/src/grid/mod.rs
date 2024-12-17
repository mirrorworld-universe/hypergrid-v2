pub mod router;
pub mod runtime;
pub mod storage;

use crate::{config::RoutingLayerConfig, NodeScaffolding};
use anyhow::Result;
use async_trait::async_trait;
use grid_node_core::{Network, NodeType};
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
pub struct Grid<N: Network> {
    router: GridRouter<N>,
    runtime: GridRuntime<N>,
    storage: GridStorage<N>,
}

impl<N: Network> Grid<N> {
    pub fn new(routing_config: RoutingLayerConfig) -> Result<Self> {
        let runtime = GridRuntime::<N>::new();

        Ok(Self {
            router: GridRouter::new(routing_config, runtime),
            // TODO: Configure GridStorage;
            storage: GridStorage::new(),
            // TODO: Configure GridRuntime;
            runtime: GridRuntime::new(),
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
