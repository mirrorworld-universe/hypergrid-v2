pub mod builder;
pub mod config;
pub mod error;
pub mod grid;

use crate::{config::RoutingLayerConfig, grid::Grid};
use anyhow::Result;
use async_trait::async_trait;
use grid_node_core::prelude::*;
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
pub enum Node<C: Cluster> {
    /// Grid Node Type
    ///
    /// Mainly responsible for sequencing incoming
    /// transactions
    ///
    Grid(Arc<Grid<C>>),
    // /// Data Node Type
    // ///
    // /// Mainly responsible for hypergrid's distributed
    // /// data availability layer
    // ///
    // Data(Arc<Data<N>>),
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
pub trait NodeScaffolding {
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
