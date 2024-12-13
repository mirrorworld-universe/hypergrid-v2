mod core;
mod traits;

use anyhow::Result;
use async_trait::async_trait;
use grid_node_solana_rpc::{
    jsonrpsee::{
        core::{RpcResult, SubscriptionResult},
        PendingSubscriptionSink,
    },
    rpc_http::SolanaRpcServer,
    rpc_pubsub::SolanaRpcPubSubServer,
    solana_rpc_client_api::config::{RpcSendTransactionConfig, RpcSimulateTransactionConfig},
};
use std::{
    marker::PhantomData,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};
use traits::*;

const DEFAULT_RPC_PORT: u16 = 1024;
const DEFAULT_NODE_IP: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
const DEFAULT_NODE_TYPE: NodeType = NodeType::Grid;

fn main() {
    let grid = SFN::<network::Solana>::new(
        DEFAULT_NODE_IP,
        DEFAULT_NODE_TYPE,
        DEFAULT_RPC_PORT,
        SFNRuntime::new(),
        SFNStorage::new(),
    );
}

//------------------------------------------
// Simple Full Node
//------------------------------------------

#[derive(Clone)]
pub struct SFN<N: Network> {
    node_ip: IpAddr,
    node_type: NodeType,
    rpc_port: u16,
    runtime: SFNRuntime<N>,
    storage: SFNStorage<N>,
}

impl<N: Network> SFN<N> {
    pub fn new(
        node_ip: IpAddr,
        node_type: NodeType,
        rpc_port: u16,
        runtime: SFNRuntime<N>,
        storage: SFNStorage<N>,
    ) -> Self {
        Self {
            node_ip,
            rpc_port,
            node_type,
            runtime,
            storage,
        }
    }
}

#[async_trait]
impl<N: Network> NodeScaffolding<N> for SFN<N> {
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
impl<N: Network> Routing<N> for SFN<N> {
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
impl<N: Network> InboundRpcHttp for SFN<N> {
    fn rpc_url(&self) -> SocketAddr {
        SocketAddr::new(self.ip(), self.port())
    }

    fn port(&self) -> u16 {
        self.rpc_port
    }
}

// SolanaRpcServer
#[async_trait]
impl<N: Network> SolanaRpcServer for SFN<N> {
    async fn send_transaction(
        &self,
        transaction: String,
        config: Option<RpcSendTransactionConfig>,
    ) -> RpcResult<String> {
        // self.runtime.process_transaction();
        println!("Transaction: {:?}", transaction);
        println!("Config: {:?}", config);
        Ok(String::new())
    }

    async fn simulate_transaction(
        &self,
        transaction: String,
        config: Option<RpcSimulateTransactionConfig>,
    ) -> RpcResult<String> {
        // self.runtime.process_transaction();
        Ok(String::new())
    }
}

//------------------------------------------
// SFNRouter
//------------------------------------------

// pub struct SFNRouter<N> {}
//
// impl<N: Network> SFNRouter<N> {
//     pub fn new() -> Self {
//         Self {}
//     }
// }

//------------------------------------------
// SFNRuntime
//------------------------------------------

#[derive(Clone)]
pub struct SFNRuntime<N: Network> {
    _network: PhantomData<N>,
}

impl<N: Network> SFNRuntime<N> {
    pub fn new() -> Self {
        Self {
            _network: Default::default(),
        }
    }
}

//------------------------------------------
// SFNStorage
//------------------------------------------

#[derive(Clone)]
pub struct SFNStorage<N: Network> {
    _network: PhantomData<N>,
}

impl<N: Network> SFNStorage<N> {
    pub fn new() -> Self {
        Self {
            _network: Default::default(),
        }
    }
}

impl<N: Network> Storage<N> for MockStorage {}
