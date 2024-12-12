use grid_node_core::Network;

/// Storage Trait.
///
/// Defines the expected storage operation
/// functions expected from different storage
/// implementations (e.g. RocksDB)
///
pub trait Storage<N: Network> {}
