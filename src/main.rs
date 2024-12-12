use libp2p::{
    core::upgrade,
    futures::StreamExt,
    mplex,
    noise::{Keypair, NoiseConfig, X25519Spec},
    swarm::{keep_alive, NetworkBehaviour, Swarm, SwarmBuilder, SwarmEvent},
    tcp::TokioTcpConfig,
    Multiaddr, PeerId, Transport,
};
use std::collections::HashSet;
use std::time::Duration;
use tokio::sync::mpsc;

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "OutEvent")]
struct MyBehaviour {
    keep_alive: keep_alive::Behaviour,
    connected_peers: HashSet<PeerId>,
}

#[derive(Debug)]
enum OutEvent {
    None,
}

impl NetworkBehaviour for MyBehaviour {
    type ConnectionHandler = keep_alive::Handler;
    type OutEvent = OutEvent;

    fn new_handler(&mut self, connection: Connection) -> Self::ConnectionHandler {
        self.keep_alive.new_handler(connection)
    }

    fn handle_established(&mut self, connection: Connection) {
        self.keep_alive.handle_established(connection);
    }

    fn handle_disconnected(&mut self, connection: Connection) {
        self.keep_alive.handle_disconnected(connection);
    }
}

impl NetworkBehaviourEvent for OutEvent {}

impl MyBehaviour {
    fn new() -> Self {
        Self {
            keep_alive: keep_alive::Behaviour::new(keep_alive::Config::default()),
            connected_peers: HashSet::new(),
        }
    }

    fn get_connected_peers(&self) -> Vec<PeerId> {
        self.connected_peers.iter().cloned().collect()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let local_key = Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("Local peer id: {:?}", local_peer_id);

    let transport = TokioTcpConfig::new()
        .nodelay(true)
        .upgrade(upgrade::Version::V1)
        .authenticate(NoiseConfig::xx(local_key).into_authenticated())
        .multiplex(mplex::MplexConfig::new())
        .boxed();

    let mut behaviour = MyBehaviour::new();
    let mut swarm = SwarmBuilder::new(transport, behaviour, local_peer_id)
        .executor(Box::new(|fut| {
            tokio::spawn(fut);
        }))
        .build();

    // Listen on all interfaces
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    // Connect to Infura's libp2p node
    let infura_address: Multiaddr = "/dns4/ipfs.infura.io/tcp/5001".parse()?;
    swarm.dial(infura_address)?;

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("Listening on {:?}", address);
            }
            SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                println!("Connected to {:?}", peer_id);
                swarm.behaviour_mut().connected_peers.insert(peer_id);
            }
            SwarmEvent::ConnectionClosed { peer_id, .. } => {
                println!("Disconnected from {:?}", peer_id);
                swarm.behaviour_mut().connected_peers.remove(&peer_id);
            }
            _ => {}
        }

        // Print current connected peers periodically
        let peers = swarm.behaviour().get_connected_peers();
        println!("Connected peers: {:?}", peers);
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}