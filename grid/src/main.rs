use clap::{crate_authors, crate_description, crate_name, crate_version, Arg, Command};

#[tokio::main]
async fn main() {
    let app_m = Command::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg_required_else_help(true)
        .subcommand(
            Command::new("start")
                .about("Grid Starter")
                .arg_required_else_help(true)
                .arg(Arg::new("MODE").long("mode").short('m').required(true)),
        )
        .get_matches();

    match app_m.subcommand() {
        Some(("start", start_m)) => {
            println!("{start_m:?}");
        }

        _ => {
            // Handled by clap `arg_required_else_help(true)`
            println!("unsupported command");
        }
    }
}

//------------------------------------------------------------
// crate::network
//------------------------------------------------------------
pub mod network {
    pub trait Cluster: Copy + Clone + Send + Sync + 'static {
        const NAME: &'static str;
    }

    //------------------------------------------------------------
    // Cluster: CanaryV0
    //------------------------------------------------------------
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub struct CanaryV0 {}
    impl Cluster for CanaryV0 {
        const NAME: &'static str = "canary-v0";
    }
}

//------------------------------------------------------------
// crate::node
//------------------------------------------------------------
pub mod node {
    use anyhow::Result;
    use jsonrpsee::server::ServerBuilder;
    use std::{marker::PhantomData, net::SocketAddr};

    pub enum Node<C: super::network::Cluster, R: Runtime<C>, P: Routing<C, R>> {
        Sequencer(Sequencer<C, R, P>),
    }

    //------------------------------------------------------------
    // Node: Sequencer
    //
    // Layers:
    // - Routing
    // - Processor
    // - Runtime
    // - Storage
    //------------------------------------------------------------
    pub struct Sequencer<C: super::network::Cluster, R: Runtime<C>, P: Routing<C, R>> {
        runtime: R,
        router: P,
        _cluster: PhantomData<C>,
    }

    impl<C: super::network::Cluster, R: Runtime<C>, P: Routing<C, R>> Sequencer<C, R, P> {
        pub fn new(runtime: R, router: P) -> Self {
            Self {
                runtime,
                router,
                _cluster: Default::default(),
            }
        }
    }

    //------------------------------------------------------------
    // Runtime
    //------------------------------------------------------------
    pub trait Runtime<C: super::network::Cluster> {}

    //------------------------------------------------------------
    // Routing
    //------------------------------------------------------------
    pub trait Routing<C: super::network::Cluster, R: Runtime<C>>: InboundRpcHttp<C, R> {}

    #[async_trait::async_trait]
    pub trait InboundRpcHttp<C: super::network::Cluster, R: Runtime<C>>:
        Clone + super::solana::SolanaRpcServer
    {
        /// Enables HTTP RPC gateways.
        async fn enable_listener(&self) -> Result<()> {
            // Handle error in Node level
            let server = ServerBuilder::default().build(self.rpc_url()).await?;
            let server_handle = server.start(self.clone().into_rpc());
            server_handle.stopped().await;
            Ok(())
        }

        /// Returns full RPC URL
        fn rpc_url(&self) -> SocketAddr;
    }
}

pub mod solana {
    use jsonrpsee::{core::RpcResult, proc_macros::rpc};
    use solana_rpc_client_api::config::RpcSendTransactionConfig;

    #[rpc(server)]
    pub trait SolanaRpc {
        // --------------------------
        // Send / Simulate
        // --------------------------

        #[method(name = "sendTransaction")]
        async fn send_transaction(
            &self,
            transaction: String,
            config: Option<RpcSendTransactionConfig>,
        ) -> RpcResult<String>;
    }
}
