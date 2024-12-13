use core::NodeType;

//------------------------------------------
// Node.
//------------------------------------------

use grid_node_router::InboundRpcHttp;

#[async_trait]
pub trait NodeScaffolding<N: Network>: Routing {
    //------------------------------------------
    // Associated Functions.
    //------------------------------------------

    /// Prepares Node before running.
    fn prepare(&self);
    /// Gracefully shuts down Node and its running services.
    fn shutdown(&self);

    //------------------------------------------
    // Asynchronous Associated Functions.
    //------------------------------------------

    /// Runs Node and initial services.
    async fn run(&self) -> Result<()>;

    //------------------------------------------
    // Getters.
    //------------------------------------------

    /// Get Node type.
    fn node_type(&self) -> NodeType;
}

//------------------------------------------
// Node Layers.
//------------------------------------------

#[async_trait]
pub trait Routing: InboundRpcHttp {
    /// Used for enabling routing protocols (e.g. TCP)
    async fn initialize_routers(&self) -> Result<()>;

    /// Used for enabling gateways / ingestion
    /// protocols (e.g. RPC).
    async fn enable_listeners(&self) -> Result<()>;
}

pub trait Processing {}

pub trait Caching {}

pub trait Storing {}

//------------------------------------------
// Node Attachments.
//------------------------------------------

/// Task spawning and management for Node / Node layers.
pub trait Spawning {}

//------------------------------------------
// Routing.
//------------------------------------------

/// Inbound RPC Gateway attached to Routing
pub trait InboundRpcHttp {}
