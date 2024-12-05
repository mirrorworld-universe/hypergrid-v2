use anyhow::Result;
use clap::{Parser, ValueEnum};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Network {
    Solana,
    Mock,
}

#[derive(Clone, Debug, Parser)]
pub struct Start {
    #[clap(long = "network")]
    pub network: Network,
}

impl Start {
    /// Starts the Grid node
    pub fn parse(self) -> Result<String> {
        println!("{:?}", self.network);
        Ok(String::new())
    }
}
