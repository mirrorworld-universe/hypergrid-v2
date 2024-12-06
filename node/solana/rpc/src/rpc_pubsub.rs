use jsonrpsee::{core::SubscriptionResult, proc_macros::rpc};

#[rpc(server)]
pub trait SolanaRpcPubSub {
    #[subscription(name = "slotSubscribe" => "slotNotification", unsubscribe="slotUnsubscribe", item=String)]
    async fn slot_subscribe(&self) -> SubscriptionResult;

    // #[subscription(name = "blockSubscribe" => "blockNotification", unsubscribe="blockUnsubscribe", item=String)]
    // async fn block_subscribe(
    //     &self,
    //     filter: RpcBlockSubscribeFilter,
    //     config: Option<RpcBlockSubscribeConfig>,
    // ) -> SubscriptionResult;
    //
    // #[subscription(name = "logsSubscribe" => "logsNotification", unsubscribe="logsUnsubscribe", item=String)]
    // async fn logs_subscribe(
    //     &self,
    //     filter: RpcTransactionLogsFilter,
    //     config: Option<RpcTransactionLogsConfig>,
    // ) -> SubscriptionResult;
    //
    // #[subscription(name = "signatureSubscribe" => "signatureNotification", unsubscribe="signatureUnsubscribe", item=String)]
    // async fn signature_subscribe(
    //     &self,
    //     signature: String,
    //     config: RpcSignatureSubscribeConfig,
    // ) -> SubscriptionResult;
    //
    // #[subscription(name = "accountSubscribe" => "accountNotification", unsubscribe="accountUnsubscribe", item=String)]
    // async fn account_subscribe(
    //     &self,
    //     account: String,
    //     config: Option<RpcAccountInfoConfig>,
    // ) -> SubscriptionResult;
    //
    // #[subscription(name = "programSubscribe" => "programNotification", unsubscribe="programUnsubscribe", item=String)]
    // async fn program_subscribe(
    //     &self,
    //     pubkey_str: String,
    //     config: Option<RpcProgramAccountsConfig>,
    // ) -> SubscriptionResult;
}
