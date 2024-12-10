use anyhow::Result;
use clap::{Parser, ValueEnum};
use grid_node::{NodeScaffolding, NodeType};
use grid_node_core::network::Solana;
use std::net::{IpAddr, Ipv4Addr};
use tokio::runtime::{self, Runtime};

const DEFAULT_RPC_PORT: u16 = 1024;
const DEFAULT_RPC_PUBSUB_PORT: u16 = 1025;
const DEFAULT_NODE_IP: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
const DEFAULT_NODE_NODE_TYPE: NodeType = NodeType::Grid;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum NetworkArg {
    Solana,
    Mock,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum NodeTypeArg {
    Grid,
}

pub type Port = u16;

#[derive(Clone, Debug, Parser)]
#[command(arg_required_else_help(true))]
pub struct Node {
    #[clap(short = 'n', long = "network")]
    pub network: Option<NetworkArg>,
    #[clap(long = "node-ip")]
    pub node_ip: Option<IpAddr>,
    #[clap(long = "node-type")]
    pub node_type: Option<NodeTypeArg>,
    #[clap(long = "rpc-port")]
    pub rpc_port: Option<Port>,
    #[clap(long = "rpc-pubsub-port")]
    pub rpc_pubsub_port: Option<Port>,
}

impl Node {
    /// Starts the Grid node
    pub fn parse(self) -> Result<String> {
        println!("Running Grid with {:?} Runtime", self.network);

        let node_ip: IpAddr = match self.node_ip {
            Some(ip) => ip,
            None => DEFAULT_NODE_IP,
        };

        let mut node_type = NodeType::Grid;

        if let Some(node_type_arg) = self.node_type {
            node_type = match node_type_arg {
                NodeTypeArg::Grid => NodeType::Grid,
            };
        };

        let rpc_port = match self.rpc_port {
            Some(port) => port,
            None => DEFAULT_RPC_PORT,
        };

        let rpc_pubsub_port = match self.rpc_pubsub_port {
            Some(port) => port,
            None => DEFAULT_RPC_PUBSUB_PORT,
        };

        let node: grid_node::Node<Solana> =
            grid_node::Node::<Solana>::new_grid(node_ip, node_type, rpc_port, rpc_pubsub_port);

        Self::runtime()?.block_on(async move {
            let grid_node::Node::Grid(grid) = node;

            tokio::spawn(async move { grid.run().await });

            std::future::pending::<()>().await;
        });

        Ok(String::new())
    }

    fn runtime() -> Result<Runtime> {
        Ok(runtime::Builder::new_multi_thread().enable_all().build()?)
    }
}
