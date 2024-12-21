use crate::{
    error::NodeError,
    grid::{router, runtime},
    Grid, Node,
};
use anyhow::{bail, Result};
use grid_node_core::prelude::*;
use std::{
    marker::PhantomData,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};

/// Forces a [`Clone`] for the reusable build
/// associated function.
///
pub trait Builder<C: Cluster>: Clone {
    /// Reusable builder associated function.
    ///
    /// Does not consume Self.
    ///
    /// Fallible for incorrect configuration cases.
    fn build(&self) -> Result<Node<C>>;
}

#[derive(Clone, Debug)]
pub struct NodeBuilder<C: Cluster> {
    _network: PhantomData<C>,
}

impl<C: Cluster> NodeBuilder<C> {
    /// Instantiate new Grid Node builder.
    pub fn grid_node() -> GridNodeBuilder<C> {
        GridNodeBuilder::<C>::new()
    }
}

//------------------------------------------
// Grid Node Builder.
//------------------------------------------

#[derive(Clone, Debug)]
pub struct GridNodeBuilder<C: Cluster> {
    routing_config: Option<router::Config>,
    runtime_config: Option<runtime::Config>,
    _network: PhantomData<C>,
}

impl<C: Cluster> GridNodeBuilder<C> {
    pub fn new() -> Self {
        Self {
            routing_config: None,
            runtime_config: None,
            _network: Default::default(),
        }
    }

    pub fn routing(mut self, node_ip: IpAddr, node_type: NodeType, rpc_port: u16) -> Self {
        let config = router::Config::new(node_ip, node_type, rpc_port);
        self.routing_config = Some(config);
        self
    }

    pub fn runtime(mut self) -> Self {
        let config = runtime::Config::new();
        self.runtime_config = Some(config);
        self
    }
}

impl<C: Cluster> Builder<C> for GridNodeBuilder<C> {
    fn build(&self) -> Result<Node<C>> {
        let routing_config = match self.routing_config.clone() {
            Some(cfg) => cfg,
            None => {
                bail!(NodeError::InvalidConfig(String::from(
                    "need a routing layer config"
                )))
            }
        };

        let runtime_config = match self.runtime_config.clone() {
            Some(cfg) => cfg,
            None => {
                bail!(NodeError::InvalidConfig(String::from(
                    "need a runtime layer config"
                )))
            }
        };

        Ok(Node::Grid(Arc::new(Grid::new(
            routing_config,
            runtime_config,
        )?)))
    }
}

//------------------------------------------
// Data Node Builder.
//------------------------------------------

impl<C: Cluster> NodeBuilder<C> {
    /// Instantiate new Data Node builder.
    pub fn data_node() -> DataNodeBuilder {
        DataNodeBuilder::new()
    }
}

#[derive(Clone, Debug)]
pub struct DataNodeBuilder;

impl DataNodeBuilder {
    pub fn new() -> Self {
        Self {}
    }
}
