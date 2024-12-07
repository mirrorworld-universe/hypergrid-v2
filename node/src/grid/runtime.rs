use grid_node_core::Network;
use grid_node_runtime::Runtime;
use std::marker::PhantomData;

//------------------------------------------
// Runtime
//------------------------------------------

#[derive(Copy, Clone, Debug)]
pub struct GridRuntime<N: Network> {
    _network: PhantomData<N>,
}

impl<N: Network> GridRuntime<N> {
    pub fn new() -> Self {
        Self {
            _network: Default::default(),
        }
    }
}

impl<N: Network> Runtime<N> for GridRuntime<N> {
    fn process_transaction(&self) {
        println!("Processing transaction...");
    }
}
