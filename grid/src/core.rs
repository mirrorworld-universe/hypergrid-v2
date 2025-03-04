use anyhow::Result;
use jsonrpsee::server::ServerBuilder;
use std::{net::SocketAddr, sync::Arc};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClusterConfig {
    name: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Cluster {
    CanaryV0(ClusterConfig),
}

impl Cluster {
    pub fn new_canary_v0(name: &str) -> Self {
        Cluster::CanaryV0(ClusterConfig { name })
    }

    pub fn get_config(&self) -> &ClusterConfig {
        match self {
            Self::CanaryV0(c) => c,
        }
    }
}

pub enum Node<R: Runtime, P: Routing> {
    Sequencer(Arc<Sequencer<R, P>>),
}

impl<R: Runtime, P: Routing> Node<R, P> {
    pub fn new_sequencer(runtime: R, router: P) -> Self {
        Self::Sequencer(Arc::new(Sequencer::new(runtime, router)))
    }
}

//------------------------------------------------------------
// Node: Sequencer
//
// Layers:
// - Routing
// - Ordering
// - Runtime
// - Storage
//------------------------------------------------------------
pub struct Sequencer<R: Runtime, P: Routing> {
    runtime: R,
    router: P<R>,
}

impl<R: Runtime, P: Routing> Sequencer<R, P> {
    pub fn new(runtime: R, router: P) -> Self {
        Self { runtime, router }
    }
}

//------------------------------------------------------------
// NodeScaffolding
//------------------------------------------------------------
#[async_trait::async_trait]
pub trait NodeScaffolding {
    async fn prepare(&self) -> Result<()>;
    async fn start(&self) -> Result<()>;
    async fn shutdown(&self) -> Result<()>;
}

//------------------------------------------------------------
// Runtime
//------------------------------------------------------------
/// Note: jsonrpsee `#[rpc(server)]` requires Send + Sync + 'static
#[async_trait::async_trait]
pub trait Runtime: Clone + Send + Sync + 'static {
    async fn process_transaction(&self) -> Result<()>;
}

//------------------------------------------------------------
// Routing
//------------------------------------------------------------
#[async_trait::async_trait]
pub trait Routing {
    async fn enable_listeners(&self) -> Result<()>;
}
