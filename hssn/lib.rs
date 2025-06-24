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

    // #[method(name = "getRegistry")]
    // async fn get_registry(&self) -> RpcResult<String>;
    //
    // #[method(name = "getRegistryInfo")]
    // async fn get_registry_info(&self, epoch: u64) -> RpcResult<String>;
}
