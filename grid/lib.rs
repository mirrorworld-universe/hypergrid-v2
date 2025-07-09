pub mod error;

use error::GridError;
use jsonrpsee::{core::RpcResult, proc_macros::rpc, server::ServerBuilder};
use solana_program_runtime::{
    loaded_programs::{BlockRelation, ForkGraph, ProgramCache, ProgramCacheEntry},
    sysvar_cache::SysvarCache,
};
use solana_rpc_client_api::config::{RpcSendTransactionConfig, RpcSimulateTransactionConfig};
use solana_sdk::{
    account::{AccountSharedData, ReadableAccount},
    clock::{Clock, Slot, UnixTimestamp},
    instruction::Instruction,
    pubkey::Pubkey,
    system_program,
    transaction::{SanitizedTransaction, Transaction},
};
use solana_svm::{
    account_loader::CheckedTransactionDetails,
    transaction_processing_callback::TransactionProcessingCallback,
    transaction_processor::{
        TransactionBatchProcessor, TransactionProcessingConfig, TransactionProcessingEnvironment,
    },
};
use std::{collections::HashMap, sync::RwLock};

pub struct GridAccountsDBConfig {}

pub struct GridAccountsDB {
    state: HashMap<Pubkey, AccountSharedData>,
}

impl GridAccountsDB {
    fn with_config(config: GridAccountsDBConfig) -> Self {
        Self {
            state: HashMap::new(),
        }
    }

    fn write_account(
        &mut self,
        pubkey: &Pubkey,
        account: AccountSharedData,
    ) -> Result<(), GridError> {
        _ = self.state.insert(*pubkey, account);
        Ok(())
    }

    fn read_account(&self, pubkey: &Pubkey) -> Result<Option<&AccountSharedData>, GridError> {
        let account = self.state.get(pubkey);
        Ok(account)
    }
}

pub struct GridConfig {}

pub struct Grid {
    accounts_db: RwLock<GridAccountsDB>,
}

impl Grid {
    fn with_config(config: GridConfig) -> Self {
        // Initialize SVM API

        Self {
            accounts_db: RwLock::new(GridAccountsDB::with_config(GridAccountsDBConfig {})),
        }
    }
}

impl TransactionProcessingCallback for Grid {
    fn account_matches_owners(&self, account: &Pubkey, owners: &[Pubkey]) -> Option<usize> {
        let Some(accounts_db) = self.accounts_db.read().ok() else {
            return None;
        };

        let Some(data) = accounts_db.state.get(account) else {
            return None;
        };

        if data.lamports() == 0 {
            None
        } else {
            owners.iter().position(|entry| data.owner() == entry)
        }
    }

    fn get_account_shared_data(&self, pubkey: &Pubkey) -> Option<AccountSharedData> {
        let Some(accounts_db) = self.accounts_db.read().ok() else {
            return None;
        };

        accounts_db.state.get(pubkey).cloned()
    }

    // Optional: Handle built-in accounts if needed.
    fn add_builtin_account(&self, _name: &str, _program_id: &Pubkey) {
        // Leave empty for now; implement if required.
    }
}

#[async_trait::async_trait]
impl SolanaRpcServer for Grid {
    async fn send_transaction(
        &self,
        transaction: String,
        config: Option<RpcSendTransactionConfig>,
    ) -> RpcResult<String> {
        Ok(String::new())
    }

    async fn simulate_transaction(
        &self,
        transaction: String,
        config: Option<RpcSimulateTransactionConfig>,
    ) -> RpcResult<String> {
        Ok(String::new())
    }
}

#[rpc(server)]
pub trait SolanaRpc {
    // --------------------------
    // Send / Simulate
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

    // // --------------------------
    // // Archival
    // // --------------------------
    //
    // #[method(name = "getBlock")]
    // async fn get_block(&self, slot: u64) -> RpcResult<Option<UiConfirmedBlock>>;
    //
    // #[method(name = "getBlocks")]
    // async fn get_blocks(
    //     &self,
    //     start_slot: Slot,
    //     config: Option<RpcBlocksConfigWrapper>,
    //     commitment: Option<CommitmentConfig>,
    // ) -> RpcResult<Vec<Slot>>;
    //
    // #[method(name = "getSignaturesForAddress")]
    // async fn get_signatures_for_address(
    //     &self,
    //     address: String,
    //     config: Option<RpcSignaturesForAddressConfig>,
    // ) -> RpcResult<Vec<RpcConfirmedTransactionStatusWithSignature>>;
    //
    // #[method(name = "getTransaction")]
    // async fn get_transaction(
    //     &self,
    //     signature_str: String,
    //     config: Option<RpcEncodingConfigWrapper<RpcTransactionConfig>>,
    // ) -> Result<Option<EncodedConfirmedTransactionWithStatusMeta>>;
    //
    // // --------------------------
    // // Cluster
    // // --------------------------
    //
    // #[method(name = "getClusterNodes")]
    // async fn get_cluster_nodes(&self) -> RpcResult<Vec<RpcContactInfo>>;
    //
    // // --------------------------
    // // Validator
    // // --------------------------
    //
    // #[method(name = "getSlot")]
    // async fn get_slot(&self, config: Option<RpcContextConfig>) -> RpcResult<Slot>;
    //
    // #[method(name = "getBlockHeight")]
    // async fn get_block_height(&self, config: Option<RpcContextConfig>) -> RpcResult<u64>;
    //
    // #[method(name = "getBlockTime")]
    // async fn get_block_time(&self, block: u64) -> RpcResult<u64>;
    //
    // #[method(name = "getFirstAvailableBlock")]
    // async fn get_first_available_block(&self) -> RpcResult<u64>;
    //
    // #[method(name = "getLatestBlockhash")]
    // async fn get_latest_blockhash(
    //     &self,
    //     config: Option<RpcContextConfig>,
    // ) -> RpcResult<RpcResponse<RpcBlockhash>>;
    //
    // #[method(name = "isBlockhashValid")]
    // async fn is_blockhash_valid(
    //     &self,
    //     blockhash: String,
    //     config: Option<IsBlockHashValidConfig>,
    // ) -> RpcResult<RpcResponse<bool>>;
    //
    // #[method(name = "getBlockCommitment")]
    // async fn get_block_commitment(
    //     &self,
    //     block: u64,
    // ) -> Result<RpcBlockCommitment<BlockCommitmentArray>>;
    //
    // #[method(name = "getRecentPerformanceSamples")]
    // async fn get_recent_performance_samples(
    //     &self,
    //     limit: Option<usize>,
    // ) -> RpcResult<Vec<RpcPerfSample>>;
    //
    // #[method(name = "getSignatureStatuses")]
    // async fn get_signature_statuses(
    //     &self,
    //     signature_strs: Vec<String>,
    //     config: Option<RpcSignatureStatusConfig>,
    // ) -> RpcResult<RpcResponse<Vec<Option<TransactionStatus>>>>;
    //
    // #[method(name = "getRecentPrioritizationFees")]
    // async fn get_recent_prioritization_fees(
    //     &self,
    //     pubkey_strs: Vec<String>,
    // ) -> RpcResult<Vec<RpcPrioritizationFee>>;
    //
    // #[method(name = "getEpochInfo")]
    // async fn get_epoch_info(&self, config: Option<RpcContextConfig>) -> RpcResult<EpochInfo>;
    //
    // #[method(name = "getLeaderSchedule")]
    // async fn get_leader_schedule(
    //     &self,
    //     slot: Option<u64>,
    //     config: Option<RpcLeaderScheduleConfig>,
    // ) -> RpcResult<Option<HashMap<String, Vec<usize>>>>;
    //
    // #[method(name = "getSlotLeaders")]
    // async fn get_slot_leaders(&self, start_slot: u64, limit: u64) -> RpcResult<Vec<Pubkey>>;
    //
    // #[method(name = "getVoteAccounts")]
    // async fn get_vote_accounts(
    //     &self,
    //     config: Option<RpcGetVoteAccountsConfig>,
    // ) -> RpcResult<RpcVoteAccountStatus>;
    //
    // // --------------------------
    // // Accounts
    // // --------------------------
    //
    // #[method(name = "getAccountInfo")]
    // async fn get_account_info(
    //     &self,
    //     pubkey_str: String,
    //     config: Option<RpcAccountInfoConfig>,
    // ) -> RpcResult<RpcResponse<Option<UiAccount>>>;
    //
    // #[method(name = "getMultipleAccounts")]
    // async fn get_multiple_accounts(
    //     &self,
    //     pubkey_strs: Vec<String>,
    //     config: Option<RpcAccountInfoConfig>,
    // ) -> RpcResult<RpcResponse<Vec<Option<UiAccount>>>>;
    //
    // #[method(name = "getProgramAccounts")]
    // async fn get_program_accounts(
    //     &self,
    //     program_id_str: String,
    //     config: Option<RpcProgramAccountsConfig>,
    // ) -> RpcResult<OptionalContext<Vec<RpcKeyedAccount>>>;
    //
    // #[method(name = "getBalance")]
    // async fn get_balance(
    //     &self,
    //     pubkey_str: String,
    //     config: Option<RpcContextConfig>,
    // ) -> RpcResult<RpcResponse<u64>>;
}

/// Disabling Fork Graph
///
/// Single sequencer (Grid) means no fork can occur
pub struct DisabledForkGraph;

impl ForkGraph for DisabledForkGraph {
    fn relationship(&self, a: Slot, b: Slot) -> BlockRelation {
        BlockRelation::Unknown
    }
}
