use anyhow::Result;
use grid_core::{GridRuntime, GridStorage};
use grid_solana_rpc::{rpc_http::SolanaRpcServer, rpc_pubsub::SolanaRpcPubSubServer};
use jsonrpsee::{
    core::{RpcResult, SubscriptionResult},
    proc_macros::rpc,
    PendingSubscriptionSink,
};
use solana_account_decoder::UiAccount;
use solana_rpc_client_api::{
    config::{
        RpcAccountInfoConfig, RpcBlocksConfigWrapper, RpcContextConfig, RpcEncodingConfigWrapper,
        RpcGetVoteAccountsConfig, RpcLeaderScheduleConfig, RpcProgramAccountsConfig,
        RpcSendTransactionConfig, RpcSignatureStatusConfig, RpcSignaturesForAddressConfig,
        RpcSimulateTransactionConfig,
    },
    response::{
        OptionalContext, Response as RpcResponse, RpcBlockhash,
        RpcConfirmedTransactionStatusWithSignature, RpcContactInfo, RpcKeyedAccount, RpcPerfSample,
        RpcPrioritizationFee, RpcVoteAccountStatus,
    },
};
use solana_sdk::{
    commitment_config::CommitmentConfig, epoch_info::EpochInfo, pubkey::Pubkey, slot_history::Slot,
};
use solana_transaction_status::{TransactionStatus, UiConfirmedBlock};
use std::collections::HashMap;
use tokio::signal;

pub struct GridNode {
    runtime: Box<dyn GridRuntime>,
    storage: Box<dyn GridStorage>,
}

impl GridNode {
    pub fn new(runtime: impl GridRuntime, storage: impl GridStorage) -> Self {
        Self {
            runtime: Box::new(runtime),
            storage: Box::new(storage),
        }
    }

    async fn start(&self) -> Result<()> {
        println!("Starting Grid...");

        tokio::select! {
            _ = signal::ctrl_c() => {
                println!("Received SIGINT");
                println!("Shutting down Grid...");
            }
        }

        Ok(())
    }
}

#[jsonrpsee::core::async_trait]
impl SolanaRpcServer for GridNode {
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

#[jsonrpsee::core::async_trait]
impl SolanaRpcPubSubServer for GridNode {
    async fn slot_subscribe(&self, pending: PendingSubscriptionSink) -> SubscriptionResult {
        Ok(())
    }
}
