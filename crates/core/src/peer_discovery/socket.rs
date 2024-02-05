use std::error::Error;
use std::net::{Ipv4Addr, SocketAddrV4};
use socket2::{Domain, Protocol, Socket, Type};
use crate::peer_discovery::constants::{DISCOVERY_ADDRESS, DISCOVERY_PORT};

pub fn create_multicast_socket() -> Result<std::net::UdpSocket, Box<dyn Error>> {
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
    socket.set_nonblocking(true)?;
    socket.join_multicast_v4(
        DISCOVERY_ADDRESS.ip(),
        address.ip()
    )?;
    return Ok(socket.into())
}