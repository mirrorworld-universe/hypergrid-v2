use crate::rpc::DietRpcServer;

use jsonrpsee::core::RpcResult;

use solana_rpc_client_api::config::{RpcSendTransactionConfig, RpcSimulateTransactionConfig};

/// Diet RPC Bridge (SVM)
///
/// Connecting the RPC interface to SVM API
///
/// Methods
///
pub struct DietRpcBridge;

#[jsonrpsee::core::async_trait]
impl DietRpcServer for DietRpcBridge {
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
