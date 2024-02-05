use tokio::net::UdpSocket;
use tokio::sync::mpsc::Receiver;
use tokio_util::udp::UdpFramed;
use crate::peer_discovery::protocol::ServerHeaderCodec;
use crate::peer_discovery::socket::{create_multicast_socket, DISCOVERY_ADDRESS};

use tokio_stream::StreamExt;
use futures_util::{SinkExt};

#[derive(Debug)]
pub struct PeerDiscoveryServer {}

impl PeerDiscoveryServer {
    pub fn new() -> PeerDiscoveryServer {
        PeerDiscoveryServer {}
    }
    pub async fn run(&mut self, mut rx_quit: Receiver<()>) {
        let socket = create_multicast_socket().unwrap();
        let tokio_socket = UdpSocket::from_std(socket).unwrap();
        let mut server = UdpFramed::new(tokio_socket, ServerHeaderCodec::new());
        tokio::select! {
            _ = async {
                while let Some(Ok((_, addr))) = server.next().await {
                    println!("received something from {addr}");
                    server.send(((), DISCOVERY_ADDRESS.into())).await.unwrap();
                }
            } => {}
            _ = rx_quit.recv() => {}
        }
    }
}