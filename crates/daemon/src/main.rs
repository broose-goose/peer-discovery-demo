use std::error::Error;
use peer_discovery_core::run_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Running server");
    run_server().await
}