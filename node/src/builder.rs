use crate::{
    config::{RoutingLayerConfig, RuntimeLayerConfig},
    error::NodeError,
    Grid, Node,
};
use anyhow::{bail, Result};
use grid_node_core::{Network, NodeType};
use std::{
    marker::PhantomData,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};

/// Forces a [`Clone`] for the reusable build
/// associated function.
///
pub trait Builder<N: Network>: Clone {
    /// Reusable builder associated function.
    ///
    /// Does not consume Self.
    ///
    /// Fallible for incorrect configuration cases.
    fn build(&self) -> Result<Node<N>>;
}
#[derive(Clone, Debug)]
pub struct NodeBuilder<N: Network> {
    _network: PhantomData<N>,
}

impl<N: Network> NodeBuilder<N> {
    pub fn new() -> Self {
        Self {
            _network: Default::default(),
        }
    }
}

//------------------------------------------
// Grid Node Builder.
//------------------------------------------

impl<N: Network> NodeBuilder<N> {
    /// Instantiate new Grid Node builder.
    pub fn grid_node(&self) -> GridNodeBuilder<N> {
        GridNodeBuilder::<N>::new()
    }
}

#[derive(Clone, Debug)]
pub struct GridNodeBuilder<N: Network> {
    routing_config: Option<RoutingLayerConfig>,
    runtime_config: Option<RuntimeLayerConfig>,
    _network: PhantomData<N>,
}

impl<N: Network> GridNodeBuilder<N> {
    pub fn new() -> Self {
        Self {
            routing_config: None,
            runtime_config: None,
            _network: Default::default(),
        }
    }

    pub fn routing(mut self, node_ip: IpAddr, node_type: NodeType, rpc_port: u16) -> Self {
        let config = RoutingLayerConfig::new(node_ip, node_type, rpc_port);
        self.routing_config = Some(config);
        self
    }

    pub fn runtime(mut self) -> Self {
        let config = RuntimeLayerConfig::new();
        self.runtime_config = Some(config);
        self
    }
}

impl<N: Network> Builder<N> for GridNodeBuilder<N> {
    fn build(&self) -> Result<Node<N>> {
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

impl<N: Network> NodeBuilder<N> {
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
