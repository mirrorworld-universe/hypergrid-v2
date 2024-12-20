use grid_node_core::Network;
use grid_node_runtime::Runtime;
use std::{marker::PhantomData, ops::Deref, sync::Arc};

use crate::config::RuntimeLayerConfig;

//------------------------------------------
// Runtime
//------------------------------------------

/// GridRuntime.
///
/// A specific Runtime implementation.
///
/// This one utilizes the SVM API.
///
/// Important Note:
/// The Struct(Arc<InnerStruct>) with impl Deref pattern
/// is used because 1 Arc for the entire struct is better
/// than having multiple Arcs among the fields and causing
/// more heap allocation per Arc<T>.
///
#[derive(Clone, Debug)]
pub(crate) struct GridRuntime<N: Network>(Arc<InnerGridRuntime<N>>);

impl<N: Network> Deref for GridRuntime<N> {
    type Target = Arc<InnerGridRuntime<N>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Debug)]
pub struct InnerGridRuntime<N: Network> {
    _network: PhantomData<N>,
}

impl<N: Network> GridRuntime<N> {
    pub fn new(config: RuntimeLayerConfig) -> Self {
        Self(Arc::new(InnerGridRuntime {
            _network: Default::default(),
        }))
    }
}

impl<N: Network> Runtime for GridRuntime<N> {
    fn process_transaction(&self) {
        println!("Processing transaction...");
    }
}
