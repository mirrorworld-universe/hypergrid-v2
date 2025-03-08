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
// Storage
//------------------------------------------------------------
#[async_trait::async_trait]
pub trait Storage: Clone + Send + Sync {
    type Key;
    type Value;
    async fn store_account(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value>;
    async fn get_account(&self, key: &Self::Key) -> Option<&Self::Value>;
}

//------------------------------------------------------------
// Routing
//------------------------------------------------------------
#[async_trait::async_trait]
pub trait Routing<R: Runtime, S: Storage>: Clone + Send + Sync {
    async fn enable_listeners(&self) -> Result<()>;
}
