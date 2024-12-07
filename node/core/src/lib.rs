/// Network Trait.
///
/// Identifies the different clusters that exist
/// within the network.
///
pub trait Network: Copy + Clone + Send + Sync + 'static {
    const TYPE: &'static str;
}
