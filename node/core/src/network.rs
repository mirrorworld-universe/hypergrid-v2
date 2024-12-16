use super::Network;

#[derive(Copy, Clone, Debug)]
pub struct Solana;

impl Network for Solana {
    const NAME: &'static str = "solana";
}

#[derive(Copy, Clone, Debug)]
pub struct Canary {}

impl Network for Canary {
    const NAME: &'static str = "canary";
}
