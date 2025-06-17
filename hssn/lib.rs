pub mod error;

pub struct HssnConfig {}

pub struct Hssn {}

impl Hssn {
    fn with_config(config: HssnConfig) -> Self {
        Self {}
    }
}

#[rpc(server)]
pub trait HssnRpc {}
