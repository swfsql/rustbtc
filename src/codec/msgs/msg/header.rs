use arrayvec::ArrayVec;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use codec::msgs::msg::commons::bytes::Bytes;
use codec::msgs::msg::commons::into_bytes::IntoBytes;
use codec::msgs::msg::commons::new_from_hex::NewFromHex;
use codec::msgs::msg::commons::params::Network;
use std;
use std::fmt;
use std::io::Cursor;

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
}

mod cmd_value {
    pub const TX: &[u8] = b"tx\0\0\0\0\0\0\0\0\0\0";
    pub const PING: &[u8] = b"ping\0\0\0\0\0\0\0\0";
    pub const PONG: &[u8] = b"pong\0\0\0\0\0\0\0\0";
    pub const VERSION: &[u8] = b"version\0\0\0\0\0";
    pub const VERACK: &[u8] = b"verack\0\0\0\0\0\0";
    pub const GETHEADERS: &[u8] = b"getheaders\0\0";
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
        };
        write!(f, "{}", s)
    }
}

impl IntoBytes for Cmd {
    fn into_bytes(&self) -> Result<Vec<u8>> {
        Ok(self.value().to_vec())
    }
}

// https://en.bitcoin.it/wiki/Protocol_documentation#tx
#[derive(Clone)]
pub struct Header {
    pub network: Network,
    pub cmd: Cmd,
    pub payload_len: i32,
    pub payloadchk: u32,
}

impl NewFromHex for Header {
    fn new<'a, I>(it: I) -> Result<Header>
    where
        I: IntoIterator<Item = &'a u8>,
    {
        let mut it = it.into_iter();
        i!("new from hex for Header");
        let aux = it.by_ref().take(4).cloned().collect::<Vec<u8>>();
        let network = Cursor::new(&aux)
            .read_u32::<LittleEndian>()
            .chain_err(cf!("Error at u32 parse for network for value {:?}", aux))?;
        let network = Network::new(network).ok_or(ff!("Error: Network Magic Number unkown"))?;
        let cmd = it.by_ref()
            .take(12)
            .cloned()
            .collect::<ArrayVec<[u8; 12]>>();
        let cmd = Cmd::new(cmd).ok_or(ff!("Error: Error when reading cmd"))?;
        let aux = it.by_ref().take(4).cloned().collect::<Vec<u8>>();
        let payload_len = Cursor::new(&aux).read_i32::<LittleEndian>().chain_err(cf!(
            "Error at i32 parse for payload_len for value {:?}",
            aux
        ))?;
        let aux = it.by_ref().take(4).cloned().collect::<Vec<u8>>();
        let payloadchk = Cursor::new(&aux)
            .read_u32::<LittleEndian>()
            .chain_err(cf!("Error at u32 parse for payloadchk for value {:?}", aux))?;
        Ok(Header {
            network,
            cmd,
            payload_len,
            payloadchk,
        })
    }
}

impl std::fmt::Debug for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        let mut s = "Message Header:\n".to_string();
        s += &format!("├ Message Network Identification: {:?}", self.network);
        s += &format!(
            "├ Message Command OP_CODE: {:?}\n",
            self.cmd.value().into_iter().collect::<Bytes>()
        );
        //str::from_utf8
        s += &format!("├ Payload Length: {}\n", self.payload_len);
        s += &format!("├ Payload Checksum: {}\n", self.payloadchk);

        write!(f, "{}", s)
    }
}

impl IntoBytes for Header {
    fn into_bytes(&self) -> Result<Vec<u8>> {
        let mut wtr = vec![];
        wtr.write_u32::<LittleEndian>(self.network.value())
            .chain_err(cf!(
                "Failure to convert network ({:?}) into byte vec",
                self.network
            ))?;

        wtr.append(&mut self.cmd.value().to_vec());
        //.chain_err(cf!("Failure to convert cmd ({}) into byte vec", self.cmd))?;

        wtr.write_i32::<LittleEndian>(self.payload_len)
            .chain_err(cf!(
                "Failure to convert payload_len ({}) into byte vec",
                self.payload_len
            ))?;
        wtr.write_u32::<LittleEndian>(self.payloadchk)
            .chain_err(cf!(
                "Failure to convert payloadchk ({}) into byte vec",
                self.payloadchk
            ))?;

        Ok(wtr)
    }
}
