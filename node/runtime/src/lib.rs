use grid_node_core::Network;

pub trait Runtime<N: Network> {
    fn process_transaction(&self);
}
