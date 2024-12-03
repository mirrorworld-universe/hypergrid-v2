use crate::rpc_ws::RpcPubSubInterfaceServer;

use jsonrpsee::{core::SubscriptionResult, PendingSubscriptionSink};

/// SVM Diet RPC PubSub
///
/// Connecting the PubSub RPC interface to SVM API
///
/// Methods
/// -
pub struct GridRpcPubSub;

#[jsonrpsee::core::async_trait]
impl RpcPubSubInterfaceServer for GridRpcPubSub {
    async fn slot_subscribe(&self, pending: PendingSubscriptionSink) -> SubscriptionResult {
        Ok(())
    }
}
