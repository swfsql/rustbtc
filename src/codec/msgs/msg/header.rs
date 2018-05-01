use std;
use std::fmt;
use arrayvec::ArrayVec;
use codec::msgs::msg::commons::new_from_hex::NewFromHex;
use codec::msgs::msg::commons::bytes::Bytes;
use codec::msgs::msg::commons::into_bytes::IntoBytes;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;

mod errors {
    error_chain!{}
}
use errors::*;

#[derive(Clone)]
pub enum Network {
    Main,
    Testnet,
    Testnet3,
    Namecoin,
}

impl Network {
    pub fn new(magic: u32) -> Option<Network> {
        match magic {
            0xD9B4BEF9 => Some(Network::Main),
            0xDAB5BFFA => Some(Network::Testnet),
            0x0709110B => Some(Network::Testnet3),
            0xFEB4BEF9 => Some(Network::Namecoin),
            _ => None,
        }
    }

    pub fn value(&self) -> u32 {
        match self {
            Main => 0xD9B4BEF9,
            Testnet => 0xDAB5BFFA,
            Testnet3 => 0x0709110B,
            Namecoin => 0xFEB4BEF9,
        }
    }
}


impl std::fmt::Debug for Network {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        let mut s = match self {
            Main => format!("Main <{:X}>\n", self.value()),
            Testnet => format!("Testnet <{:X}>\n", self.value()),
            Testnet3 => format!("Testnet3 <{:X}>\n", self.value()),
            Namecoin => format!("Namecoin <{:X}>\n", self.value()),
        };
        write!(f, "{}", s)
    }
}

impl IntoBytes for Network {
    fn into_bytes(&self) -> Result<Vec<u8>> {
        let mut wtr = vec![];
        wtr.write_u32::<LittleEndian>(self.value())
            .chain_err(|| format!("Failure to convert network ({}) into byte vec", self.value()))?;
        Ok(wtr)
    }
}

#[derive(Clone)]
pub enum Cmd {
    Tx,
    Ping,
    Pong,
    Version,
    Verack,
}

const TX: &[u8] =  b"tx\0\0\0\0\0\0\0\0\0\0";
const PING: &[u8] = b"ping\0\0\0\0\0\0\0\0";
const PONG: &[u8] = b"pong\0\0\0\0\0\0\0\0";
const VERSION: &[u8] = b"version\0\0\0\0\0";
const VERACK: &[u8] = b"verack\0\0\0\0\0\0";

impl Cmd {
    pub fn new(arrayvec: ArrayVec<[u8; 12]>) -> Option<Cmd> {
        match arrayvec.as_slice() {
            TX => Some(Cmd::Tx),
            PING => Some(Cmd::Ping),
            PONG => Some(Cmd::Pong),
            VERSION => Some(Cmd::Version),
            VERACK => Some(Cmd::Verack),
            _ => None,
        }
    }

    pub fn value(&self) -> ArrayVec<[u8; 12]> {
        let bytes = match *self {
            Cmd::Tx => TX,
            Cmd::Ping => PING,
            Cmd::Pong => PONG,
            Cmd::Version => VERSION,
            Cmd::Verack => VERACK,
        };
        bytes.iter().cloned().collect::<ArrayVec<[u8; 12]>>()
    }
}

impl std::fmt::Debug for Cmd {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        let mut s = match *self {
            Cmd::Tx => format!("Cmd::Tx <{:?}>\n", self.value()),
            Cmd::Ping => format!("Cmd::Ping <{:?}>\n", self.value()),
            Cmd::Pong => format!("Cmd::Pong <{:?}>\n", self.value()),
            Cmd::Version => format!("Cmd::Version <{:?}>\n", self.value()),
            Cmd::Verack => format!("Cmd::Verack <{:?}>\n", self.value()),
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
    fn new(it: &mut std::vec::IntoIter<u8>) -> Result<Header> {
        i!("new from hex for Header");
        let aux = it.take(4).collect::<Vec<u8>>();
        let network = Cursor::new(&aux).read_u32::<LittleEndian>().chain_err(|| {
            format!(
                "(Msg::header) Error at u32 parse for network for value {:?}",
                aux
            )
        })?;
        let network = Network::new(network)
            .ok_or("Error: Network Magic Number unkown")?;
        let cmd = it.take(12).collect::<ArrayVec<[u8; 12]>>();
        let cmd = Cmd::new(cmd)
            .ok_or("(Msg::header) Error: Error when reading cmd")?;
        let aux = it.take(4).collect::<Vec<u8>>();
        let payload_len = Cursor::new(&aux).read_i32::<LittleEndian>().chain_err(|| {
            format!(
                "(Msg::header) Error at i32 parse for payload_len for value {:?}",
                aux
            )
        })?;
        let aux = it.take(4).collect::<Vec<u8>>();
        let payloadchk = Cursor::new(&aux).read_u32::<LittleEndian>().chain_err(|| {
            format!(
                "(Msg::header) Error at u32 parse for payloadchk for value {:?}",
                aux
            )
        })?;
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
            .chain_err(|| format!("Failure to convert network ({:?}) into byte vec", self.network))?;

        wtr.append(&mut self.cmd.value().to_vec());
            //.chain_err(|| format!("Failure to convert cmd ({}) into byte vec", self.cmd))?;

        wtr.write_i32::<LittleEndian>(self.payload_len)
            .chain_err(|| format!("Failure to convert payload_len ({}) into byte vec", self.payload_len))?;
        wtr.write_u32::<LittleEndian>(self.payloadchk)
            .chain_err(|| format!("Failure to convert payloadchk ({}) into byte vec", self.payloadchk))?;

        Ok(wtr)
    }
}
