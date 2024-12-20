use grid_node_core::prelude::*;

/// Storage Trait.
///
/// Defines the expected storage operation
/// functions expected from different storage
/// implementations (e.g. RocksDB)
///
pub trait Storage<C: Cluster> {}
