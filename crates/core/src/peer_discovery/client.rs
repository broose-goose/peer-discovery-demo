

use std::error::Error;
use std::time::Duration;
use tokio::net::UdpSocket;
use tokio::time;
use tokio_util::udp::UdpFramed;
use crate::peer_discovery::protocol::ClientHeaderCodec;
use crate::peer_discovery::socket::{create_multicast_socket, DISCOVERY_ADDRESS};

use tokio_stream::StreamExt;
use futures_util::{SinkExt};

// #[derive(Debug)]
// pub struct PeerDiscoveryClient {
//     timout_in_ms: Duration,
//     iterations: Option<u32>,
//     discovered_server: Option<Ipv4Addr>
// }
//
// impl PeerDiscoveryClient {
//     pub fn new(timeout_in_ms: Option<u64>, iterations: Option<u32>) -> PeerDiscoveryClient {
//         let timeout = Duration::from_millis(timeout_in_ms.unwrap_or(5000));
//         return PeerDiscoveryClient{
//             timout_in_ms: timeout,
//             iterations,
//             discovered_server: None
//         }
//     }
//     pub fn start() {}
//     pub fn cancel() {}
//     async fn run() {
//
//     }
// }

pub async fn run_client(timeout_in_ms: Option<u64>, iterations: Option<u32>) -> Result<(), Box<dyn Error>> {
    let timeout = Duration::from_millis(timeout_in_ms.unwrap_or(5000));
    let max_iterations = iterations.unwrap_or(10);
    let socket = create_multicast_socket()?;
    let tokio_socket = UdpSocket::from_std(socket.into())?;
    let mut client = UdpFramed::new(tokio_socket, ClientHeaderCodec::new());
    for _ in 0..max_iterations {
        client.send(((), DISCOVERY_ADDRESS.into())).await?;
        match time::timeout(timeout, client.next()).await {
            Ok(Some(Ok((_, addr)))) => {
                println!("Server responded at {addr}");
                return Ok(())
            },
            _ => println!("Didn't hear back from server, trying again"),
        }
    }
    return Ok(())
}