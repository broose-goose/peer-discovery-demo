
mod protocol;

use std::error::Error;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::time::Duration;
use socket2::{Domain, Protocol, Socket, Type};
use tokio::net::UdpSocket;
use tokio::time;
use tokio_util::udp::UdpFramed;
use protocol::{ClientHeaderCodec, ServerHeaderCodec};
use tokio_stream::StreamExt;
use futures_util::{SinkExt};


const DISCOVERY_HOST: Ipv4Addr = Ipv4Addr::new(228, 255, 255, 255);
const DISCOVERY_PORT: u16 = 50962;
const DISCOVERY_ADDRESS: SocketAddrV4 = SocketAddrV4::new(DISCOVERY_HOST, DISCOVERY_PORT);

fn create_multicast_socket() -> Result<std::net::UdpSocket, Box<dyn Error>> {
    let socket = Socket::new(
        Domain::IPV4,
        Type::DGRAM,
        Some(Protocol::UDP)
    )?;
    socket.set_reuse_address(true)?;
    let address = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), DISCOVERY_PORT);
    let addr = address.into();
    socket.bind(&addr)?;
    socket.set_multicast_loop_v4(true)?;
    socket.join_multicast_v4(
        DISCOVERY_ADDRESS.ip(),
        address.ip()
    )?;
    return Ok(socket.into())
}

pub async fn run_server() -> Result<(), Box<dyn Error>> {
    let socket = create_multicast_socket()?;
    let tokio_socket = UdpSocket::from_std(socket.into())?;
    let mut server = UdpFramed::new(tokio_socket, ServerHeaderCodec::new());
    while let Some(Ok((_, addr))) = server.next().await {
        println!("received something");
        server.send(((), addr)).await?;
    }
    return Ok(())
}

pub async fn run_client(timeout_in_ms: Option<u64>, iterations: Option<u32>) -> Result<(), Box<dyn Error>> {
    let timeout = Duration::from_millis(timeout_in_ms.unwrap_or(5000));
    let max_iterations = iterations.unwrap_or(10);
    let socket = create_multicast_socket()?;
    let tokio_socket = UdpSocket::from_std(socket.into())?;
    let mut client = UdpFramed::new(tokio_socket, ClientHeaderCodec::new());
    for _ in 0..max_iterations {
        client.send(((), DISCOVERY_ADDRESS.into())).await?;
        match time::timeout(timeout, client.next()).await {
            Ok(Some(Ok((_, addr)))) => println!("Server responded at {addr}"),
            _ => println!("Didn't hear back from server, trying again"),
        }
    }
    return Ok(())
}