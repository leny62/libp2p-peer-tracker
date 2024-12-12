# Libp2p Peer Tracker - Detailed Technical Description

## Overview
The libp2p-peer-tracker is a Rust application that demonstrates basic peer-to-peer networking capabilities using the libp2p framework. It maintains connections with other peers in the network and tracks their connection status in real-time.


## Project Structure
```
src/
  └── main.rs
```

## Solution Description
### Dependencies
The solution uses several key dependencies:
- `libp2p`: Core networking stack with features for TCP, Tokio async runtime, noise encryption, multiplexing, and Kademlia DHT
- `tokio`: Async runtime for handling concurrent operations
- `futures`: Utilities for working with asynchronous code
- `idna`: International domain names handling
- `url`: URL parsing and manipulation

### Core Functionality

1. **Network Behavior Implementation**
   - Custom `MyBehaviour` struct that implements `NetworkBehaviour`
   - Tracks connected peers using a `HashSet`
   - Implements keep-alive functionality to maintain connections

2. **Transport Configuration**
   - Uses TCP transport with noise encryption for secure communications
   - Implements multiplexing using mplex protocol
   - Configures authentication using XX handshake pattern

3. **Connection Management**
   - Listens on all network interfaces
   - Connects to Infura's libp2p node as a bootstrap peer
   - Tracks peer connections and disconnections
   - Periodically reports connected peers

## Technical Details

### Initialization
- Generates ED25519 keypair for node identity
- Creates a TCP transport with:
  - Node-to-node delay disabled
  - Upgrade protocol version V1
  - Noise protocol for encryption
  - Mplex for stream multiplexing

### Network Behavior
The custom behavior tracks:
- Keep-alive connections
- Connected peer set
- Connection events

### Event Handling
Processes several types of network events:
- New listen address establishment
- Peer connection establishment
- Peer connection closure
- Regular peer list updates

## Usage and Implementation Notes

1. The system automatically:
   - Generates a unique peer ID on startup
   - Listens on an available TCP port
   - Connects to Infura's IPFS node
   - Maintains a list of connected peers
   - Prints connection status updates

2. The implementation provides:
   - Real-time peer tracking
   - Connection event logging
   - Periodic peer list updates
   - Automatic connection maintenance

## Security Considerations

The implementation includes several security features:
- Noise protocol encryption for all communications
- XX handshake pattern for mutual authentication
- Unique ED25519 keypair for node identity

This peer tracking system provides a foundation for building more complex P2P applications, with robust connection handling and peer management capabilities.

## Technologies Used
- Rust

## Getting Started

### Prerequisites
- Rust installed on your system
- Cargo (Rust's package manager)

### Installation
1. Clone the repository
```bash
git clone https://github.com/leny62/libp2p-peer-tracker
```

2. Navigate to the project directory
```bash
cd libp2p-peer-tracker
```

3. Build the project
```bash
cargo build
```

4. Run the project
```bash
cargo run
```

## Testing
```bash
cargo test
```

## Contributing
This is a personal project for PLDG application, but suggestions and feedback are welcome.

## License
This project is open-source and available under the MIT License.

## Author
Leny Pascal IHIRWE

## Acknowledgments
- The web3 community for inspiration

---
