use crate::{config::RoutingLayerConfig, grid::runtime::GridRuntime};
use anyhow::Result;
use async_trait::async_trait;
use grid_node_core::prelude::*;
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
pub(crate) struct GridRouter<C: Cluster>(Arc<InnerGridRouter<C>>);

impl<C: Cluster> Deref for GridRouter<C> {
    type Target = Arc<InnerGridRouter<C>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Debug)]
pub struct InnerGridRouter<C: Cluster> {
    node_ip: IpAddr,
    rpc_port: u16,
    node_type: NodeType,
    runtime: GridRuntime<C>,
    _network: PhantomData<C>,
}

impl<C: Cluster> GridRouter<C> {
    pub fn new(config: RoutingLayerConfig, runtime: GridRuntime<C>) -> Self {
        Self(Arc::new(InnerGridRouter {
            node_ip: config.node_ip,
            node_type: config.node_type,
            rpc_port: config.rpc_port,
            _network: Default::default(),
            runtime,
        }))
    }
}

//------------------------------------------
// Routing
//------------------------------------------

// Routing
#[async_trait]
impl<C: Cluster> Routing for GridRouter<C> {
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
impl<C: Cluster> InboundRpcHttp for GridRouter<C> {
    fn rpc_url(&self) -> SocketAddr {
        SocketAddr::new(self.node_ip, self.rpc_port)
    }

    fn port(&self) -> u16 {
        self.rpc_port
    }
}

// SolanaRpcServer
#[async_trait]
impl<C: Cluster> SolanaRpcServer for GridRouter<C> {
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
// impl<C: Cluster> InboundRpcPubSub for GridRouter<C> {}
//
// // SolanaRpcPubSubServer
// #[async_trait]
// impl<C: Cluster> SolanaRpcPubSubServer for GridRouter<C> {
//     async fn slot_subscribe(&self, pending: PendingSubscriptionSink) -> SubscriptionResult {
//         Ok(())
//     }
// }
