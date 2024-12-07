use grid_node_core::Network;

/// Runtime Trait.
///
/// Defines the expected data and functions from
/// different runtime implementations (e.g. SVM API).
///
pub trait Runtime<N: Network> {
    fn process_transaction(&self);
}
