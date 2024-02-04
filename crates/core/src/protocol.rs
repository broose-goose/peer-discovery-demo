use std::fmt::{Display, Formatter};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use tokio_util::bytes::{Buf, BufMut, BytesMut};
use tokio_util::codec::{Decoder, Encoder};
use crate::protocol::MessageType::{AnnounceServer, FindServer};

#[derive(Clone, Copy, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(i32)]
pub enum MessageType {
    FindServer = 100,
    AnnounceServer = 101,
}


#[derive(Clone, Copy, Debug)]
#[repr(packed(8))]
pub struct Header {
    message_type: MessageType,
}

impl Header {
    fn new(message_type: MessageType) -> Header {
        Header {
            message_type
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::protocol::{Header, MessageType};

    #[test]
    fn i_know_how_packing_works() {
        assert_eq!(std::mem::size_of::<MessageType>(), 4);
        assert_eq!(std::mem::size_of::<Header>(), 4);
    }
}

#[derive(Debug)]
pub enum ProtocolError {
    IO(std::io::Error),
}

impl From<std::io::Error> for ProtocolError {
    fn from(err: std::io::Error) -> ProtocolError {
        ProtocolError::IO(err)
    }
}

impl Display for ProtocolError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "You fkd up")
    }
}

impl std::error::Error for ProtocolError {}

pub struct ServerHeaderCodec {}

impl ServerHeaderCodec {
    pub fn new() -> ServerHeaderCodec {
        ServerHeaderCodec {}
    }
}

impl Encoder<()> for ServerHeaderCodec {
    type Error = ProtocolError;
    fn encode(&mut self, _event: (), dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.reserve(4);
        dst.put_i32(AnnounceServer.into());
        Ok(())
    }
}

impl Decoder for ServerHeaderCodec {
    type Item = Header;
    type Error = ProtocolError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Header>, Self::Error> {
        /*
         * let's just assume our packets are never fragmented... they hella shouldn't be
         *     as it's just 4 bytes...
         */
        if src.len() != 4 {
            return Ok(None)
        }
        return match src.get_i32().try_into() {
            Ok(message_type @ FindServer) => Ok(Some(Header::new(message_type))),
            _ => Ok(None)
        }
    }
}

pub struct ClientHeaderCodec {}

impl ClientHeaderCodec {
    pub fn new() -> ClientHeaderCodec {
        ClientHeaderCodec {}
    }
}

impl Encoder<()> for ClientHeaderCodec {
    type Error = ProtocolError;
    fn encode(&mut self, _event: (), dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.reserve(4);
        dst.put_i32(FindServer.into());
        Ok(())
    }
}

impl Decoder for ClientHeaderCodec {
    type Item = Header;
    type Error = ProtocolError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Header>, Self::Error> {
        /*
         * let's just assume our packets are never fragmented... they hella shouldn't be
         *     as it's just 4 bytes...
         */
        if src.len() != 4 {
            return Ok(None)
        }
        return match src.get_i32().try_into() {
            Ok(message_type @ AnnounceServer) => Ok(Some(Header::new(message_type))),
            _ => Ok(None)
        }
    }
}