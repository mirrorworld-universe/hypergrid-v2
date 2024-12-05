pub mod transaction;

use anyhow::Result;
use async_trait::async_trait;

/*
 * @dev
 * - 'static for dynamic dispatch with Box<>
 * - Send + Sync because we expect these impls
 * to be able to parallel process
 */

#[async_trait]
pub trait GridRuntime: Send + Sync + 'static {
    // from GridLoader
    fn fetch_account();
    fn settle_account();
    // from GridProcessor
    fn process_transaction();
    fn process_transaction_batch();
}

#[async_trait]
pub trait GridGateway: Send + Sync + 'static {
    async fn start_http_server(&self) -> Result<()>;
    async fn start_ws_server(&self) -> Result<()>;
}

#[async_trait]
pub trait GridStorage: Send + Sync + 'static {}
