use jsonrpsee::{core::SubscriptionResult, proc_macros::rpc};

/// Diet RPC PubSub
///
/// Websocket (PubSub) Interface for Grid RPC
///
/// Methods
/// -
#[rpc(server)]
pub trait DietRpcPubSub {
    #[subscription(name = "slotSubscribe" => "slotNotification", unsubscribe="slotUnsubscribe", item=String)]
    async fn slot_subscribe(&self) -> SubscriptionResult;
}
