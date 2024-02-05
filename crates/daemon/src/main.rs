use std::error::Error;
use tokio::sync::mpsc::channel;
use peer_discovery_core::PeerDiscoveryServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Running server");
    let mut server = PeerDiscoveryServer::new();
    let (tx_quit, rx_quit) = channel(32);
    tokio::spawn(async move { server.run(rx_quit).await; });
    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    tx_quit.send(()).await?;
    return Ok(())
}