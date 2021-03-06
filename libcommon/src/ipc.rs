use bytes::{Buf, BufMut, BytesMut};
use tokio_util::codec::{Decoder, Encoder};

pub use super::error::IpcError;

pub const MAGIC: i16 = ((b'1' as i16) << 8) as i16 + b'w' as i16;
pub const HEADER_SIZE: usize = 8;

#[derive(Debug, Clone)]
pub enum IpcMessage {
    ClientHello,
    ServerHello,
    ClientBye,
    RequestAuthentication(String, String),
}

#[derive(Debug)]
pub struct IpcMessageCodec;

type DecodeResult = Result<Option<(IpcMessage, usize)>, IpcError>;

impl IpcMessage {
    pub fn message_type(&self) -> &[u8; 2] {
        match *self {
            IpcMessage::ClientHello => b"CH",
            IpcMessage::ServerHello => b"SH",
            IpcMessage::ClientBye => b"CB",
            IpcMessage::RequestAuthentication(ref _a, ref _b) => b"RA",
        }
    }
}

impl Encoder<IpcMessage> for IpcMessageCodec {
    type Error = IpcError;

    fn encode(&mut self, msg: IpcMessage, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let len = match msg {
            IpcMessage::ClientHello | IpcMessage::ServerHello | IpcMessage::ClientBye => 0,
            IpcMessage::RequestAuthentication(ref user, ref secret) => {
                user.len() + secret.len() + 8
            }
            _ => return Err(IpcError::UnknownMessageType),
        };

        dst.reserve(HEADER_SIZE + len);
        dst.put_i16(MAGIC);
        dst.extend(msg.message_type());
        dst.put_u32(len as u32);

        if let IpcMessage::RequestAuthentication(ref user, ref secret) = msg {
            dst.put_u32(user.len() as u32);
            dst.extend(user.as_bytes());
            dst.put_u32(secret.len() as u32);
            dst.extend(secret.as_bytes());
        }

        Ok(())
    }
}

impl Decoder for IpcMessageCodec {
    type Item = IpcMessage;
    type Error = IpcError;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        match decode(buf, 0) {
            Ok(None) => Ok(None),
            Ok(Some((item, pos))) => {
                buf.advance(pos);
                Ok(Some(item))
            }
            Err(e) => Err(e),
        }
    }
}

fn decode(buf: &mut BytesMut, idx: usize) -> DecodeResult {
    let length = buf.len();
    if length <= idx {
        return Ok(None);
    }

    if length < HEADER_SIZE {
        return Err(IpcError::HeaderTooShort);
    }

    let magic = &buf[idx..idx + 2];
    if magic[0] != b'1' || magic[1] != b'w' {
        return Err(IpcError::InvalidMagic);
    }

    let message_type = &buf[idx + 2..idx + 4];
    match message_type {
        b"CH" => Ok(Some((IpcMessage::ClientHello, idx + HEADER_SIZE))),
        b"SH" => Ok(Some((IpcMessage::ServerHello, idx + HEADER_SIZE))),
        b"CB" => Ok(Some((IpcMessage::ClientBye, idx + HEADER_SIZE))),
        _ => Err(IpcError::UnknownMessageType),
    }
}
