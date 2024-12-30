use grid_node_core::prelude::*;
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
pub(crate) struct GridStorage<C: Cluster>(Arc<InnerGridStorage<C>>);

impl<C: Cluster> Deref for GridStorage<C> {
    type Target = Arc<InnerGridStorage<C>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Debug)]
pub struct InnerGridStorage<C: Cluster> {
    _network: PhantomData<C>,
}

impl<C: Cluster> GridStorage<C> {
    pub fn new() -> Self {
        Self(Arc::new(InnerGridStorage {
            _network: Default::default(),
        }))
    }
}

// impl<C: Cluster> Storage<C> for GridStorage<C> {}
