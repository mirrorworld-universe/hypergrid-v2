use libp2p::{
    gossipsub::{Behaviour as GossipsubBehavior, Event as GossipsubEvent},
    identify::{Behaviour as IdentifyBehaviour, Event as IdentifyEvent},
    kad::{
        store::MemoryStore as KadInMemory, Behaviour as KadBehaviour, Event as KadEvent,
        RoutingUpdate,
    },
    swarm::{behaviour, NetworkBehaviour},
    Multiaddr, PeerId,
};

#[derive(NetworkBehaviour)]
// #[behaviour(to_swarm = "Event")]
pub(crate) struct Node {
    gossipsub: GossipsubBehavior,
    identify: IdentifyBehaviour,
    kad: KadBehaviour<KadInMemory>,
}

impl Node {
    pub fn new(
        gossipsub: GossipsubBehavior,
        identify: IdentifyBehaviour,
        kad: KadBehaviour<KadInMemory>,
    ) -> Self {
        Self {
            gossipsub,
            identify,
            kad,
        }
    }

    pub fn add_address(&mut self, peer_id: &PeerId, addr: Multiaddr) -> RoutingUpdate {
        self.kad.add_address(peer_id, addr)
    }
}
