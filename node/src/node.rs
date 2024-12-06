use anyhow::Result;
use std::marker::PhantomData;
use std::sync::Arc;

pub trait Network {
    fn get_network_id() -> String;
}

pub trait AccountStorage {
    const NAME: &'static str;
    fn store_account() -> Result<()>;
}

pub trait LedgerStorage {
    const NAME: &'static str;
    fn store_bucket() -> Result<()>;
}

pub struct KVStore {}

impl AccountStorage for KVStore {
    const NAME: &'static str = "KVStore";

    fn store_account() -> Result<()> {
        Ok(())
    }
}

impl LedgerStorage for KVStore {
    const NAME: &'static str = "KVStore";

    fn store_bucket() -> Result<()> {
        Ok(())
    }
}

pub struct GridStorage<N: Network, A: AccountStorage, L: LedgerStorage> {
    _account: PhantomData<A>,
    _ledger: PhantomData<L>,
}

impl<A: AccountStorage, L: LedgerStorage> GridStorage<A, L> {
    fn store() -> Result<()> {
        L::store_bucket()?;
        A::store_account()?;
        Ok(())
    }
}

pub struct Grid<N: Network, A: AccountStorage, L: LedgerStorage> {
    storage: GridStorage<N, A, L>,
    _phantom: PhantomData<N>,
}

pub enum Node<N: Network> {
    /// A Grid is a full node with an rpc, processor and storage
    Grid(Arc<Grid<N, KVStore, KVStore>>),
    /// A Storage is a node with read rpc gateway and storage
    Storage(Arc<GridStorage<N, KVStore, KVStore>>),
}
