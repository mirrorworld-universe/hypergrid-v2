pub mod error;

use jsonrpsee::{core::RpcResult, proc_macros::rpc, server::ServerBuilder};

pub struct HssnConfig {}

pub struct Hssn {}

impl Hssn {
    fn with_config(config: HssnConfig) -> Self {
        Self {}
    }
}

#[rpc(server)]
pub trait HssnRpc {
    #[method(name = "getAttentionReport")]
    async fn get_attention_report(&self, epoch: u64) -> RpcResult<String>;
}
