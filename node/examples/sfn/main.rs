use async_trait::async_trait;
use grid_node::*;
use grid_node_core::*;
use grid_node_router::*;
use grid_node_runtime::*;
use grid_node_solana_rpc::*;
use grid_node_storage::*;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;

const DEFAULT_RPC_PORT: u16 = 1024;
const DEFAULT_NODE_IP: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
const DEFAULT_NODE_TYPE: NodeType = NodeType::Grid;

fn main() {
    let grid = Grid::new(
        DEFAULT_NODE_IP,
        DEFAULT_NODE_TYPE,
        DEFAULT_RPC_PORT,
        GridRuntime::new(),
        GridStorage::new(),
    );
}

//------------------------------------------
// Grid
//------------------------------------------

pub struct Grid {
    node_ip: IpAddr,
    node_type: NodeType,
    rpc_port: u16,
    runtime: GridRuntime,
    storage: GridStorage,
}

impl Grid {
    pub fn new(
        node_ip: IpAddr,
        rpc_port: u16,
        node_type: NodeType,
        runtime: GridRuntime,
        storage: GridStorage,
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
impl<N: Network> rpc_http::SolanaRpcServer for Grid<N> {
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

//------------------------------------------
// GridRouter
//------------------------------------------

pub struct GridRouter {}

impl GridRouter {
    pub fn new() -> Self {
        Self {}
    }
}

//------------------------------------------
// GridRuntime
//------------------------------------------

pub struct GridRuntime {}

impl GridRuntime {
    pub fn new() -> Self {
        Self {}
    }
}

//------------------------------------------
// GridStorage
//------------------------------------------

pub struct GridStorage {}

impl GridStorage {
    pub fn new() -> Self {
        Self {}
    }
}
