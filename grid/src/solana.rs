use crate::core::{Cluster, Routing, Runtime};
use anyhow::Result;
use jsonrpsee::{core::RpcResult, proc_macros::rpc, server::ServerBuilder};
use solana_rpc_client_api::config::RpcSendTransactionConfig;
use std::sync::Arc;

//------------------------------------------------------------
// Runtime
//------------------------------------------------------------

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SolanaSvmRuntime;

impl SolanaSvmRuntime {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl Runtime for SolanaSvmRuntime {
    async fn process_transaction(&self) -> Result<()> {
        println!("processed");
        Ok(())
    }
}

//------------------------------------------------------------
// Routing
//------------------------------------------------------------

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SolanaSvmRoutingConfig {
    rpc_url: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SolanaSvmRouting<R: Runtime> {
    rpc_http: SolanaInboundRpcHttp,
    runtime: Arc<R>,
}

impl<R: Runtime> SolanaSvmRouting<R> {
    pub fn new(config: SolanaSvmRoutingConfig, runtime: R) -> Self {
        Self {
            rpc_http: SolanaInboundRpcHttp::new(config.rpc_url),
        }
    }
}

#[async_trait::async_trait]
impl Routing for SolanaSvmRouting {
    async fn enable_listeners(&self) -> Result<()> {
        self.rpc_http.start_rpc_http().await?;
        println!("enabling listeners");
        Ok(())
    }
}

//------------------------------------------------------------
// Routing Layers
//------------------------------------------------------------

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SolanaInboundRpcHttp {
    rpc_url: &'static str,
}

impl SolanaInboundRpcHttp {
    pub fn new(rpc_url: &str) -> Self {
        Self { rpc_url }
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
