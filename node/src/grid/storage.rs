use grid_node_core::Network;
use grid_node_storage::Storage;
use std::{marker::PhantomData, ops::Deref, sync::Arc};

//------------------------------------------
// Runtime
//------------------------------------------

/// GridStorage.
///
/// A specific Storage implementation.
///
/// Important Note:
/// The Struct(Arc<InnerStruct>) with impl Deref pattern
/// is used because 1 Arc for the entire struct is better
/// than having multiple Arcs among the fields and causing
/// more heap allocation per Arc<T>.
///
#[derive(Clone, Debug)]
pub(crate) struct GridStorage<N: Network>(Arc<InnerGridStorage<N>>);

impl<N: Network> Deref for GridStorage<N> {
    type Target = Arc<InnerGridStorage<N>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Debug)]
pub struct InnerGridStorage<N: Network> {
    _network: PhantomData<N>,
}

impl<N: Network> GridStorage<N> {
    pub fn new() -> Self {
        Self(Arc::new(InnerGridStorage {
            _network: Default::default(),
        }))
    }
}

impl<N: Network> Storage<N> for GridStorage<N> {}
