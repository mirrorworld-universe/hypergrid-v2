use grid_core::GridGateway;
use grid_rpc::{SolanaRpcPubSubServer, SolanaRpcServer};
use jsonrpsee::{
    core::{RpcResult, SubscriptionResult},
    PendingSubscriptionSink,
};
use solana_rpc_client_api::config::{RpcSendTransactionConfig, RpcSimulateTransactionConfig};

pub struct SolanaGateway {
    rpc_http: SolanaGatewayRpc,
    rpc_pubsub: SolanaGatewayRpcPubSub,
}

impl GridGateway for SolanaGateway {
    async fn start_http_server(&self) -> Result<()> {
        let rpc = self.rpc_http.into_rpc();
        Ok(())
    }
    async fn start_ws_server(&self) -> Result<()> {
        let rpc = self.rpc_pubsub.into_rpc();
        Ok(())
    }
}

struct SolanaGatewayRpc;

#[jsonrpsee::core::async_trait]
impl SolanaRpcServer for SolanaGatewayRpc {
    // --------------------------
    // Send / Simulate
    // --------------------------

    async fn send_transaction(
        &self,
        transaction: String,
        config: Option<RpcSendTransactionConfig>,
    ) -> RpcResult<String> {
        Ok(String::from(""))
    }

    async fn simulate_transaction(
        &self,
        transaction: String,
        config: Option<RpcSimulateTransactionConfig>,
    ) -> RpcResult<String> {
        Ok(String::from(""))
    }
}

struct SolanaGatewayRpcPubSub;

#[jsonrpsee::core::async_trait]
impl SolanaRpcPubSubServer for SolanaGatewayRpcPubSub {
    async fn slot_subscribe(&self, pending: PendingSubscriptionSink) -> SubscriptionResult {
        Ok(())
    }
}
