use jsonrpsee::{core::RpcResult, proc_macros::rpc};

use solana_rpc_client_api::config::{RpcSendTransactionConfig, RpcSimulateTransactionConfig};

/// Diet RPC
///
/// HTTP Interface for Grid RPC
///
/// Methods
/// -
#[rpc(server)]
pub trait DietRpc {
    // --------------------------
    // Transaction Processing
    // --------------------------

    #[method(name = "sendTransaction")]
    async fn send_transaction(
        &self,
        transaction: String,
        config: Option<RpcSendTransactionConfig>,
    ) -> RpcResult<String>;

    #[method(name = "simulateTransaction")]
    async fn simulate_transaction(
        &self,
        transaction: String,
        config: Option<RpcSimulateTransactionConfig>,
    ) -> RpcResult<String>;
}
