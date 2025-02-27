use anyhow::Result;
use jsonrpsee::server::ServerBuilder;
use std::{marker::PhantomData, net::SocketAddr};

pub trait Cluster: Copy + Clone + Send + Sync + 'static {
    const NAME: &'static str;
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct CanaryV0 {}
impl Cluster for CanaryV0 {
    const NAME: &'static str = "canary-v0";
}

pub enum Node<C: Cluster, R: Runtime<C>, P: Routing<C>> {
    Sequencer(Sequencer<C, R, P>),
}

impl<C: Cluster, R: Runtime<C>, P: Routing<C>> Node<C, R, P> {
    pub fn new_sequencer(runtime: R, router: P) -> Self {
        Self::Sequencer(Sequencer::new(runtime, router))
    }
}

//------------------------------------------------------------
// Node: Sequencer
//
// Layers:
// - Routing
// - Processor
// - Runtime
// - Storage
//------------------------------------------------------------
pub struct Sequencer<C: Cluster, R: Runtime<C>, P: Routing<C>> {
    runtime: R,
    router: P,
    _cluster: PhantomData<C>,
}

impl<C: Cluster, R: Runtime<C>, P: Routing<C>> Sequencer<C, R, P> {
    pub fn new(runtime: R, router: P) -> Self {
        Self {
            runtime,
            router,
            _cluster: Default::default(),
        }
    }
}

//------------------------------------------------------------
// Runtime
//------------------------------------------------------------
/// Note: jsonrpsee `#[rpc(server)]` requires Send + Sync + 'static
#[async_trait::async_trait]
pub trait Runtime<C: Cluster>: Clone + Send + Sync + 'static {
    async fn process_transaction(&self) -> Result<()>;
}

//------------------------------------------------------------
// Routing
//------------------------------------------------------------
#[async_trait::async_trait]
pub trait Routing<C: Cluster> {
    async fn enable_listeners(&self) -> Result<()>;
}
