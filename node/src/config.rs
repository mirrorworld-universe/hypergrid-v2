use crate::grid::runtime::GridRuntime;
use grid_node_core::{Network, NodeType};
use std::net::IpAddr;

pub struct RoutingLayerConfig {
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
