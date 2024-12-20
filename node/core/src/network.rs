/// Cluster Trait.
///
/// Identifies the different clusters that exist
/// within the network.
///
pub trait Cluster: Copy + Clone + Send + Sync + 'static {
    const NAME: &'static str;
}

#[derive(Copy, Clone, Debug)]
pub struct CanaryV0 {}

impl Cluster for CanaryV0 {
    const NAME: &'static str = "canary";
}
