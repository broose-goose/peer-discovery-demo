use std::net::{Ipv4Addr, SocketAddrV4};

const DISCOVERY_HOST: Ipv4Addr = Ipv4Addr::new(228, 255, 255, 255);
pub const DISCOVERY_PORT: u16 = 50962;
pub const DISCOVERY_ADDRESS: SocketAddrV4 = SocketAddrV4::new(DISCOVERY_HOST, DISCOVERY_PORT);