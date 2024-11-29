use crate::rpc_http::DietRpcServer;

use jsonrpsee::core::RpcResult;

use solana_rpc_client_api::config::{RpcSendTransactionConfig, RpcSimulateTransactionConfig};

/// SVM Diet RPC
///
/// Connecting the RPC interface to SVM API
///
/// Methods
/// -
pub struct SvmDietRpc;

#[jsonrpsee::core::async_trait]
impl DietRpcServer for SvmDietRpc {
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
