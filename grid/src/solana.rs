use crate::core::{Cluster, Routing, Runtime};
use anyhow::Result;
use jsonrpsee::{core::RpcResult, proc_macros::rpc, server::ServerBuilder};
use solana_rpc_client_api::config::RpcSendTransactionConfig;
use std::{marker::PhantomData, net::SocketAddr};

pub struct Solana<C: Cluster> {
    rpc_http: SolanaInboundRpcHttp,
    _cluster: PhantomData<C>,
}

impl<C: Cluster> Solana<C> {
    pub fn new() -> Self {
        Self {
            _cluster: Default::default(),
        }
    }
}

#[async_trait::async_trait]
impl<C: Cluster> Runtime<C> for Solana<C> {
    async fn process_transaction(&self) -> Result<()> {
        println!("processed");
        Ok(())
    }
}

#[async_trait::async_trait]
impl<C: Cluster> Routing<C> for Solana<C> {
    async fn enable_listeners(&self) -> Result<()> {
        self.rpc_http.start_rpc_http().await?;
        println!("enabling listeners");
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct SolanaInboundRpcHttp<C: Cluster, R: Runtime<C>> {
    rpc_url: SocketAddr,
    runtime: R,
    _cluster: PhantomData<C>,
}

impl<C: Cluster, R: Runtime<C>> SolanaInboundRpcHttp<C, R> {
    pub fn new(rpc_url: SocketAddr, runtime: R) -> Self {
        Self {
            rpc_url,
            runtime,
            _cluster: Default::default(),
        }
    }

    /// Enables HTTP RPC gateways.
    async fn start_rpc_http(&self) -> Result<()> {
        // Handle error in Node level
        let server = ServerBuilder::default().build(self.rpc_url).await?;
        let server_handle = server.start(self.clone().into_rpc());
        server_handle.stopped().await;
        Ok(())
    }
}

#[async_trait::async_trait]
impl<C: Cluster, R: Runtime<C>> SolanaRpcServer for SolanaInboundRpcHttp<C, R> {
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
