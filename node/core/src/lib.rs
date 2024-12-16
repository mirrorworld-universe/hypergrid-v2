pub mod network;

/// Network Trait.
///
/// Identifies the different clusters that exist
/// within the network.
///
pub trait Network: Copy + Clone + Send + Sync + 'static {
    const NAME: &'static str;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NodeType {
    Grid,
}
