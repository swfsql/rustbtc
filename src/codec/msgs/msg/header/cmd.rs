use arrayvec::ArrayVec;
//use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
//use codec::msgs::msg::commons::bytes::Bytes;
use codec::msgs::msg::commons::into_bytes::IntoBytes;
//use codec::msgs::msg::commons::new_from_hex::NewFromHex;
//use codec::msgs::msg::commons::params::Network;
use std;
use std::fmt;
//use std::io::Cursor;

mod errors {
    error_chain!{}
}
use errors::*;

#[derive(Clone)]
pub enum Cmd {
    Tx,
    Ping,
    Pong,
    Version,
    Verack,
    GetHeaders,
    SendHeaders,
    Headers,
    GetAddr,
    Addr,
    GetData,
    Block,
    Inv,
    NotFound,
}

mod cmd_value {
    pub const TX: &[u8] = b"tx\0\0\0\0\0\0\0\0\0\0";
    pub const PING: &[u8] = b"ping\0\0\0\0\0\0\0\0";
    pub const PONG: &[u8] = b"pong\0\0\0\0\0\0\0\0";
    pub const VERSION: &[u8] = b"version\0\0\0\0\0";
    pub const VERACK: &[u8] = b"verack\0\0\0\0\0\0";
    pub const GETHEADERS: &[u8] = b"getheaders\0\0";
    pub const SENDHEADERS: &[u8] = b"sendheaders\0";
    pub const HEADERS: &[u8] = b"headers\0\0\0\0\0";
    pub const GETADDR: &[u8] = b"getaddr\0\0\0\0\0";
    pub const ADDR: &[u8] = b"addr\0\0\0\0\0\0\0\0";
    pub const BLOCK: &[u8] = b"block\0\0\0\0\0\0\0";
    pub const GETDATA: &[u8] = b"getdata\0\0\0\0\0";
    pub const INV: &[u8] = b"add\0\0\0\0\0\0\0\0\0";
    pub const NOTFOUND: &[u8] = b"notfound\0\0\0\0";
}

impl Cmd {
    pub fn new(arrayvec: ArrayVec<[u8; 12]>) -> Option<Cmd> {
        match arrayvec.as_slice() {
            cmd_value::TX => Some(Cmd::Tx),
            cmd_value::PING => Some(Cmd::Ping),
            cmd_value::PONG => Some(Cmd::Pong),
            cmd_value::VERSION => Some(Cmd::Version),
            cmd_value::VERACK => Some(Cmd::Verack),
            cmd_value::GETHEADERS => Some(Cmd::GetHeaders),
            cmd_value::SENDHEADERS => Some(Cmd::SendHeaders),
            cmd_value::HEADERS => Some(Cmd::Headers),
            cmd_value::GETADDR => Some(Cmd::GetAddr),
            cmd_value::ADDR => Some(Cmd::Addr),
            cmd_value::GETDATA => Some(Cmd::GetData),
            cmd_value::INV => Some(Cmd::Inv),
            cmd_value::BLOCK => Some(Cmd::Block),
            cmd_value::NOTFOUND => Some(Cmd::NotFound),
            _ => None,
        }
    }

    pub fn value(&self) -> ArrayVec<[u8; 12]> {
        let bytes = match *self {
            Cmd::Tx => cmd_value::TX,
            Cmd::Ping => cmd_value::PING,
            Cmd::Pong => cmd_value::PONG,
            Cmd::Version => cmd_value::VERSION,
            Cmd::Verack => cmd_value::VERACK,
            Cmd::GetHeaders => cmd_value::GETHEADERS,
            Cmd::SendHeaders => cmd_value::SENDHEADERS,
            Cmd::Headers => cmd_value::HEADERS,
            Cmd::GetAddr => cmd_value::GETADDR,
            Cmd::Addr => cmd_value::ADDR,
            Cmd::GetData => cmd_value::GETDATA,
            Cmd::Block => cmd_value::BLOCK,
            Cmd::Inv => cmd_value::INV,
            Cmd::NotFound => cmd_value::NOTFOUND,
        };
        bytes.iter().cloned().collect::<ArrayVec<[u8; 12]>>()
    }
}

impl std::fmt::Debug for Cmd {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        let s = match *self {
            Cmd::Tx => format!("Cmd::Tx <{:?}>\n", self.value()),
            Cmd::Ping => format!("Cmd::Ping <{:?}>\n", self.value()),
            Cmd::Pong => format!("Cmd::Pong <{:?}>\n", self.value()),
            Cmd::Version => format!("Cmd::Version <{:?}>\n", self.value()),
            Cmd::Verack => format!("Cmd::Verack <{:?}>\n", self.value()),
            Cmd::GetHeaders => format!("Cmd::GetHeaders <{:?}>\n", self.value()),
            Cmd::SendHeaders => format!("Cmd::SendHeaders <{:?}>\n", self.value()),
            Cmd::Headers => format!("Cmd::Headers <{:?}>\n", self.value()),
            Cmd::GetAddr => format!("Cmd::GetAddr <{:?}>\n", self.value()),
            Cmd::Addr => format!("Cmd::Addr <{:?}>\n", self.value()),
            Cmd::GetData => format!("Cmd::GetData <{:?}>\n", self.value()),
            Cmd::Inv => format!("Cmd::Inv <{:?}>\n", self.value()),
            Cmd::Block => format!("Cmd::Block <{:?}>\n", self.value()),
            Cmd::NotFound => format!("Cmd::NotFound <{:?}>\n", self.value()),
        };
        write!(f, "{}", s)
    }
}

impl IntoBytes for Cmd {
    fn into_bytes(&self) -> Result<Vec<u8>> {
        Ok(self.value().to_vec())
    }
}
