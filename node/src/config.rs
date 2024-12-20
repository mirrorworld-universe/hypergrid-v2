//! Using Layer Configs to prevent multiple argument
//! changes when modifying configuration parameters
//! for layers
use crate::grid::runtime::GridRuntime;
use grid_node_core::prelude::*;
use std::net::IpAddr;

//------------------------------------------
// Routing Layer Config
//------------------------------------------

#[derive(Clone, Debug)]
pub(crate) struct RoutingLayerConfig {
    pub node_ip: IpAddr,
    pub node_type: NodeType,
    pub rpc_port: u16,
}

impl RoutingLayerConfig {
    pub fn new(node_ip: IpAddr, node_type: NodeType, rpc_port: u16) -> Self {
        Self {
            node_ip,
            node_type,
            rpc_port,
        }
    }
}

//------------------------------------------
// Runtime Layer Config
//------------------------------------------

#[derive(Clone, Debug)]
pub(crate) struct RuntimeLayerConfig {}

impl RuntimeLayerConfig {
    pub fn new() -> Self {
        Self {}
    }
}
