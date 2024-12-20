use grid_node_core::prelude::*;
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
pub(crate) struct GridRuntime<C: Cluster>(Arc<InnerGridRuntime<C>>);

impl<C: Cluster> Deref for GridRuntime<C> {
    type Target = Arc<InnerGridRuntime<C>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Debug)]
pub struct InnerGridRuntime<C: Cluster> {
    _network: PhantomData<C>,
}

impl<C: Cluster> GridRuntime<C> {
    pub fn new(config: RuntimeLayerConfig) -> Self {
        Self(Arc::new(InnerGridRuntime {
            _network: Default::default(),
        }))
    }
}

impl<C: Cluster> Runtime for GridRuntime<C> {
    fn process_transaction(&self) {
        println!("Processing transaction...");
    }
}
