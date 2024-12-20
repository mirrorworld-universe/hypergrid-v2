use grid_node_core::prelude::*;

/// Runtime Trait.
///
/// Defines the expected data and functions from
/// different runtime implementations (e.g. SVM API).
///
pub trait Runtime {
    fn process_transaction(&self);
}
