use crate::{
    core::{Cluster, Routing, Runtime, Storage},
    mock::MockStorage,
    NodeScaffolding,
};
use anyhow::Result;
use jsonrpsee::{core::RpcResult, proc_macros::rpc, server::ServerBuilder};
use solana_rpc_client_api::config::RpcSendTransactionConfig;
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    ops::Deref,
    sync::Arc,
};

pub enum Node {
    Grid(Arc<Grid<SolanaSvmRuntime, MockStorage, SolanaSvmRouting<SolanaSvmRuntime, MockStorage>>>),
}

impl Node {
    pub fn new_grid(router_config: SolanaSvmRoutingConfig) -> Self {
        let runtime = SolanaSvmRuntime::new();
        let storage = MockStorage::new();
        let router = SolanaSvmRouting::new(router_config, runtime.clone(), storage.clone());
        Self::Grid(Arc::new(Grid::new(runtime, storage, router)))
    }
}

//------------------------------------------------------------
// Node: Grid
//
// Layers:
// - Routing
// - Ordering
// - Runtime
// - Storage
//
//------------------------------------------------------------
pub struct Grid<R: Runtime, S: Storage, P: Routing<R, S>> {
    runtime: R,
    storage: S,
    router: P,
}

impl<R: Runtime, S: Storage, P: Routing<R, S>> Grid<R, S, P> {
    pub fn new(runtime: R, storage: S, router: P) -> Self {
        Self {
            runtime,
            router,
            storage,
        }
    }
}

#[async_trait::async_trait]
impl<R: Runtime, S: Storage, P: Routing<R, S>> NodeScaffolding for Grid<R, S, P> {
    async fn start(&self) -> Result<()> {
        self.router.enable_listeners().await?;
        Ok(())
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

//------------------------------------------------------------
// Runtime
//------------------------------------------------------------

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SolanaSvmRuntime(Arc<InnerSolanaSvmRuntime>);

impl SolanaSvmRuntime {
    pub fn new() -> Self {
        Self(Arc::new(InnerSolanaSvmRuntime {}))
    }
}

#[async_trait::async_trait]
impl Runtime for SolanaSvmRuntime {
    async fn process_transaction(&self) -> Result<()> {
        println!("Processing!");
        Ok(())
    }
}

impl Deref for SolanaSvmRuntime {
    type Target = Arc<InnerSolanaSvmRuntime>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InnerSolanaSvmRuntime;

//------------------------------------------------------------
// Routing
//------------------------------------------------------------

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SolanaSvmRoutingConfig {
    pub rpc_http_url: IpAddr,
    pub rpc_http_port: u16,
}

impl SolanaSvmRoutingConfig {
    pub fn new(rpc_http_url: &str, rpc_http_port: u16) -> Self {
        Self {
            rpc_http_url: rpc_http_url
                .parse()
                .expect("rpc_http_url: invalid ip address"),
            rpc_http_port,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SolanaSvmRouting<R: Runtime, S: Storage>(Arc<InnerSolanaSvmRouting<R, S>>);

impl<R: Runtime, S: Storage> SolanaSvmRouting<R, S> {
    pub fn new(config: SolanaSvmRoutingConfig, runtime: R, storage: S) -> Self {
        Self(Arc::new(InnerSolanaSvmRouting {
            runtime,
            storage,
            rpc_http: SolanaInboundRpcHttp::new(config.rpc_http_url, config.rpc_http_port),
        }))
    }

    pub fn runtime(&self) -> &R {
        &self.runtime
    }
}

#[async_trait::async_trait]
impl<R: Runtime, S: Storage> Routing<R, S> for SolanaSvmRouting<R, S> {
    async fn enable_listeners(&self) -> Result<()> {
        self.rpc_http.start_rpc_http().await?;
        println!("enabling listeners");
        Ok(())
    }
}

impl<R: Runtime, S: Storage> Deref for SolanaSvmRouting<R, S> {
    type Target = Arc<InnerSolanaSvmRouting<R, S>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InnerSolanaSvmRouting<R: Runtime, S: Storage> {
    runtime: R,
    storage: S,
    rpc_http: SolanaInboundRpcHttp,
}

//------------------------------------------------------------
// Routing Layers
//------------------------------------------------------------

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SolanaInboundRpcHttp {
    rpc_url: SocketAddr,
}

impl SolanaInboundRpcHttp {
    pub fn new(url: IpAddr, port: u16) -> Self {
        Self {
            rpc_url: SocketAddr::new(url, port),
        }
    }

    pub async fn start_rpc_http(&self) -> Result<()> {
        // Handle error in Node level
        let server = ServerBuilder::default().build(self.rpc_url).await?;
        let server_handle = server.start(self.clone().into_rpc());
        server_handle.stopped().await;
        Ok(())
    }
}

#[async_trait::async_trait]
impl SolanaRpcServer for SolanaInboundRpcHttp {
    async fn send_transaction(
        &self,
        transaction: String,
        config: Option<RpcSendTransactionConfig>,
    ) -> RpcResult<String> {
        let result = format!("{transaction} {config:?}");
        println!("{result}");
        Ok(result)
    }
}

#[rpc(server)]
pub trait SolanaRpc {
    // --------------------------
    // Send / Simulate
    // --------------------------

    #[method(name = "sendTransaction")]
    async fn send_transaction(
        &self,
        transaction: String,
        config: Option<RpcSendTransactionConfig>,
    ) -> RpcResult<String>;
}
