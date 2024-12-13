use crate::grid::runtime::GridRuntime;
use anyhow::Result;
use async_trait::async_trait;
use grid_node_core::{Network, NodeType};
use grid_node_router::{InboundRpcHttp, Routing};
use grid_node_runtime::Runtime;
use grid_node_solana_rpc::{
    jsonrpsee::core::RpcResult,
    rpc_http::SolanaRpcServer,
    solana_rpc_client_api::{
        config::{
            RpcAccountInfoConfig, RpcBlocksConfigWrapper, RpcContextConfig,
            RpcEncodingConfigWrapper, RpcGetVoteAccountsConfig, RpcLeaderScheduleConfig,
            RpcProgramAccountsConfig, RpcSendTransactionConfig, RpcSignatureStatusConfig,
            RpcSignaturesForAddressConfig, RpcSimulateTransactionConfig,
        },
        response::{
            OptionalContext, Response as RpcResponse, RpcBlockhash,
            RpcConfirmedTransactionStatusWithSignature, RpcContactInfo, RpcKeyedAccount,
            RpcPerfSample, RpcPrioritizationFee, RpcVoteAccountStatus,
        },
    },
};
use std::{
    future::Future,
    net::{IpAddr, SocketAddr},
};
use std::{marker::PhantomData, ops::Deref, sync::Arc};

//------------------------------------------
// Router
//------------------------------------------

/// GridRouter.
///
/// A specific Routing implementation.
/// Important Note:
/// The Struct(Arc<InnerStruct>) with impl Deref pattern
/// is used because 1 Arc for the entire struct is better
/// than having multiple Arcs among the fields and causing
/// more heap allocation per Arc<T>.
///
#[derive(Clone, Debug)]
pub struct GridRouter<N: Network>(Arc<InnerGridRouter<N>>);

impl<N: Network> Deref for GridRouter<N> {
    type Target = Arc<InnerGridRouter<N>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Debug)]
pub struct InnerGridRouter<N: Network> {
    node_ip: IpAddr,
    rpc_port: u16,
    node_type: NodeType,
    runtime: GridRuntime<N>,
    _network: PhantomData<N>,
}

impl<N: Network> GridRouter<N> {
    pub fn new(
        node_ip: IpAddr,
        node_type: NodeType,
        rpc_port: u16,
        runtime: GridRuntime<N>,
    ) -> Self {
        Self(Arc::new(InnerGridRouter {
            node_ip,
            node_type,
            rpc_port,
            runtime,
            _network: Default::default(),
        }))
    }
}

//------------------------------------------
// Routing
//------------------------------------------

// Routing
#[async_trait]
impl<N: Network> Routing for GridRouter<N> {
    /// Enable all Routing listeners
    async fn enable_listeners(&self) -> Result<()> {
        self.enable_listener().await?;
        Ok(())
    }

    fn ip(&self) -> IpAddr {
        self.node_ip
    }

    fn node_type(&self) -> NodeType {
        self.node_type
    }
}

// InboundRpcHttp
#[async_trait]
impl<N: Network> InboundRpcHttp for GridRouter<N> {
    fn rpc_url(&self) -> SocketAddr {
        SocketAddr::new(self.node_ip, self.rpc_port)
    }

    fn port(&self) -> u16 {
        self.rpc_port
    }
}

// SolanaRpcServer
#[async_trait]
impl<N: Network> SolanaRpcServer for GridRouter<N> {
    async fn send_transaction(
        &self,
        transaction: String,
        config: Option<RpcSendTransactionConfig>,
    ) -> RpcResult<String> {
        self.runtime.process_transaction();
        println!("Transaction: {:?}", transaction);
        println!("Config: {:?}", config);
        Ok(String::new())
    }

    async fn simulate_transaction(
        &self,
        transaction: String,
        config: Option<RpcSimulateTransactionConfig>,
    ) -> RpcResult<String> {
        self.runtime.process_transaction();
        Ok(String::new())
    }
}

// // InboundRpcPubSub
// impl<N: Network> InboundRpcPubSub for GridRouter<N> {}
//
// // SolanaRpcPubSubServer
// #[async_trait]
// impl<N: Network> SolanaRpcPubSubServer for GridRouter<N> {
//     async fn slot_subscribe(&self, pending: PendingSubscriptionSink) -> SubscriptionResult {
//         Ok(())
//     }
// }
