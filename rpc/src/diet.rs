use jsonrpsee::{core::RpcResult, proc_macros::rpc};

use solana_rpc_client_api::config::RpcSendTransactionConfig;

#[rpc(server)]
pub trait DietRpc {
    #[method(name = "sendTransaction")]
    async fn send_transaction(
        &self,
        transaction: String,
        send_transaction_config: Option<RpcSendTransactionConfig>,
    ) -> RpcResult<String>;
}
