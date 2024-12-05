use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    account::{AccountSharedData, ReadableAccount},
    pubkey::Pubkey,
};
use solana_svm::transaction_processing_callback::TransactionProcessingCallback;

pub struct SvmLoader {
    rpc_client: RpcClient,
}

impl SvmLoader {
    pub fn new(rpc_client: RpcClient) -> Self {
        Self { rpc_client }
    }
}

impl TransactionProcessingCallback for SvmLoader {
    /// Get Account from L1
    ///
    /// Hoists L1 accounts into the SVM API representation of an
    /// [`solana_sdk::account::Account`], an [`AccountSharedData`]
    fn get_account_shared_data(&self, pubkey: &Pubkey) -> Option<AccountSharedData> {
        // ReadableAccount Trait enables AccountSharedData to be converted from Account
        // See: https://docs.rs/solana-account/2.1.4/solana_account/trait.ReadableAccount.html
        let account: AccountSharedData = self.rpc_client.get_account(pubkey).ok()?.into();
        Some(account)
    }

    /// Check Ownership
    ///
    /// Evaluate if an account is owned by one of the [`Pubkey`]
    /// listed in the owners slice
    ///
    /// Returns
    /// - Some(usize) (Index of Match)
    /// - None (No Match)
    fn account_matches_owners(&self, account: &Pubkey, owners: &[Pubkey]) -> Option<usize> {
        self.get_account_shared_data(account)
            .and_then(|account| owners.iter().position(|key| account.owner().eq(key)))
    }
}
