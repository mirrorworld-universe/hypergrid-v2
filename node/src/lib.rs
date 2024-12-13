pub mod grid;

use crate::grid::Grid;
use anyhow::Result;
use async_trait::async_trait;
use grid_node_core::{Network, NodeType};
use grid_node_router::Routing;
use std::{net::IpAddr, sync::Arc};

//------------------------------------------
// Node
//------------------------------------------

/// Node Enum.
///
/// Lists the available Node types.
///
#[derive(Debug)]
pub enum Node<N: Network> {
    /// Grid Node Type
    ///
    /// Mainly responsible for sequencing incoming
    /// transactions
    ///
    Grid(Arc<Grid<N>>),
}

impl<N: Network> Node<N> {
    pub fn new_grid(
        node_ip: IpAddr,
        node_type: NodeType,
        rpc_port: u16,
        rpc_pubsub_port: u16,
    ) -> Self {
        Self::Grid(Arc::new(Grid::new(node_ip, node_type, rpc_port).unwrap()))
    }
}

/// NodeScaffolding Trait.
///
/// Defines the behaviors expected from a crude Node.
/// - Setup
/// - Startup
/// - Task Scheduling
/// - Graceful Teardown
/// - Shutdown
/// - Node Getters
///
/// Import Note:
/// A Node is expected to have Routing, because what is a node
/// without routing?
#[async_trait]
pub trait NodeScaffolding<N: Network> {
    //------------------------------------------
    // Associated Functions
    //------------------------------------------

    /// Prepares Node before running.
    fn prepare(&self);
    /// Gracefully shuts down Node and its running services.
    fn shutdown(&self);

    //------------------------------------------
    // Asynchronous Associated Functions
    //------------------------------------------

    /// Runs Node and initial services.
    async fn run(&self) -> Result<()>;

    //------------------------------------------
    // Getters
    //------------------------------------------

    /// Get Node type.
    fn node_type(&self) -> NodeType;
}
