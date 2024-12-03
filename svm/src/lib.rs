use solana_client::rpc_client::RpcClient;
use solana_sdk::{account::AccountSharedData, pubkey::Pubkey};
use solana_svm::transaction_processing_callback::TransactionProcessingCallback;
use std::collections::HashMap;

/*
 * Steps to have a working Runtime SVM
 * 1. Define Processing Callback
 * 2. Define Processing Environment
 * 3. (Optional) Configure Processor */

// TransactionProcessingEnvironment
pub struct RuntimeConfig {}

// TransactionProcessingConfig
pub struct ProcessorConfig {}

pub struct AccountMigrator {
    rpc_client: RpcClient,
}

impl AccountMigrator {
    pub fn new(rpc_client: RpcClient) -> Self {
        Self { rpc_client }
    }
}

impl TransactionProcessingCallback for AccountMigrator {
    fn get_account_shared_data(&self, pubkey: &Pubkey) -> Option<AccountSharedData> {
        None
    }

    fn account_matches_owners(&self, account: &Pubkey, owners: &[Pubkey]) -> Option<usize> {
        None
    }

    /*
     * Optionally, implement
     */
    // fn add_builtin_account(&self, _name: &str, _program_id: &Pubkey) {}
    //
    // fn inspect_account(&self, _address: &Pubkey, _account_state: AccountState, _is_writable: bool) {}
}
