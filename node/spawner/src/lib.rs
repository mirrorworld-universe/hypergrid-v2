use grid_node_core::Network;
use std::future::Future;

/// Spawning Trait.
///
/// Defines a trait that enables the spawning of
/// long running asynchronous tasks.
///
/// Good for enabling Node layers to spawn services,
/// daemons, jobs, etc.
pub trait Spawning<N: Network> {
    /// Spawns a task with the given future.
    ///
    /// Used specifically for spawning long-running tasks.
    ///
    fn spawn<T: Future<Output = ()> + Send + 'static>(&self, future: T);
}
