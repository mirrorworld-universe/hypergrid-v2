#![cfg(feature = "tester")]

use anyhow::Result;
use clap::{Parser, ValueEnum};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum TestCommand {
    SendTransaction,
}

#[derive(Clone, Debug, Parser)]
pub struct Tester {
    #[clap(long = "test-command")]
    pub test_command: TestCommand,
}

impl Tester {
    /// Starts the Grid node
    pub fn parse(self) -> Result<String> {
        match self.test_command {
            TestCommand::SendTransaction => {}
        }

        Ok(String::new())
    }
}
