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
pub trait GridRuntime: Send + Sync + 'static {}

#[async_trait]
pub trait GridStorage: Send + Sync + 'static {}
