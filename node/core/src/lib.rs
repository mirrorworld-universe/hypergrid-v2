/// Network Trait.
///
/// Identifies the different clusters that exist
/// within the network.
///
pub trait Network: Copy + Clone + Send + Sync + 'static {
    const NAME: &'static str;
}

pub mod network {
    use super::*;

    #[derive(Copy, Clone, Debug)]
    pub struct Solana;
    impl Network for Solana {
        const NAME: &'static str = "solana";
    }
}
