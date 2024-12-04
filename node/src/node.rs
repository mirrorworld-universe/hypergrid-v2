use anyhow::Result;
use grid_core::{GridGateway, GridRuntime, GridStorage};
use tokio::signal;

pub struct GridNode {
    gateway: Box<dyn GridGateway>,
    runtime: Box<dyn GridRuntime>,
    storage: Box<dyn GridStorage>,
}

impl GridNode {
    pub fn new(
        gateway: impl GridGateway,
        runtime: impl GridRuntime,
        storage: impl GridStorage,
    ) -> Self {
        Self {
            gateway: Box::new(gateway),
            runtime: Box::new(runtime),
            storage: Box::new(storage),
        }
    }

    async fn start(&self) -> Result<()> {
        tokio::select! {
            _ = signal::ctrl_c() => {
                println!("Received SIGINT");
                println!("Shutting down Grid...");
            }

            _ = self.gateway.start_http_server() => {}
        }

        Ok(())
    }
}
