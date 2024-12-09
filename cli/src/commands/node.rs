use anyhow::Result;
use clap::{Parser, ValueEnum};
use grid_node::NodeScaffolding;
use grid_node_core::network::Solana;
use tokio::runtime::{self, Runtime};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Network {
    Solana,
    Mock,
}

#[derive(Clone, Debug, Parser)]
pub struct Node {
    #[clap(long = "network")]
    pub network: Network,
}

impl Node {
    /// Starts the Grid node
    pub fn parse(self) -> Result<String> {
        println!("Running Grid with {:?} Runtime", self.network);
        let node: grid_node::Node<Solana> = grid_node::Node::<Solana>::new_grid();

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
