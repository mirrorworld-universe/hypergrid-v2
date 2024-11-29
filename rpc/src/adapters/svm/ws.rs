use crate::rpc_ws::DietRpcPubSubServer;

use jsonrpsee::core::SubscriptionResult;

/// SVM Diet RPC PubSub
///
/// Connecting the PubSub RPC interface to SVM API
///
/// Methods
/// -
pub struct SvmDietRpcPubSub;

#[jsonrpsee::core::async_trait]
impl DietRpcPubSubServer for SvmDietRpcPubSub {
    async fn slot_subscribe(&self) -> SubscriptionResult {
        Ok(())
    }
}
