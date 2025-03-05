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
    pub fn new_canary_v0(name: &'static str) -> Self {
        Cluster::CanaryV0(ClusterConfig { name })
    }

    pub fn get_config(&self) -> &ClusterConfig {
        match self {
            Self::CanaryV0(c) => c,
        }
    }
}

//------------------------------------------------------------
// NodeScaffolding
//------------------------------------------------------------
#[async_trait::async_trait]
pub trait NodeScaffolding {
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
