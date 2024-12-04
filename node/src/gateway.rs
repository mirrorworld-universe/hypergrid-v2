use grid_core::GridGateway;
use grid_rpc::{SolanaRpcPubSubServer, SolanaRpcServer};

pub struct SolanaGatewayRpc;

impl SolanaRpcServer for SolanaGatewayRpc {}

pub struct SolanaGatewayRpcPubSub;

impl SolanaRpcPubSubServer for SolanaGatewayRpcPubSub {}

pub struct SolanaGateway {
    rpc_http: SolanaGatewayRpc,
    rpc_pubsub: SolanaGatewayRpcPubSub,
}

impl GridGateway for SolanaGateway {
    async fn start_http_server(&self) -> Result<()> {
        let rpc = self.rpc_http.into_rpc();
        Ok(())
    }
    async fn start_ws_server(&self) -> Result<()> {
        let rpc = self.rpc_pubsub.into_rpc();
        Ok(())
    }
}
