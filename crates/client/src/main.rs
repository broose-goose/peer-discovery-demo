use std::error::Error;
use peer_discovery_core::run_client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Running client");
    run_client(None, None).await
}
