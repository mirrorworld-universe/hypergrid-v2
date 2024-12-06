pub mod grid;

use crate::grid::Grid;
use async_trait::async_trait;
use grid_node_core::Network;
use grid_node_router::Routing;

//------------------------------------------
// Node
//------------------------------------------

pub enum Node<N: Network> {
    Grid(Grid<N>),
}

impl<N: Network> Node<N> {
    pub fn new_grid() -> Self {
        let node_type = NodeType::Grid;
        Self::Grid(Grid::new(node_type).unwrap())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NodeType {
    Grid,
}

#[async_trait]
pub trait NodeInterface<N: Network>: Routing<N> {
    fn node_type(&self) -> NodeType;
    async fn shutdown(&self);
}
