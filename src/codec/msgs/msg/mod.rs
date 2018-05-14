use byteorder::{LittleEndian, ReadBytesExt};
use codec::msgs::msg::commons::into_bytes::IntoBytes;
use codec::msgs::msg::commons::new_from_hex::NewFromHex;
use std;
use std::fmt;
// use codec::msgs::msg::commons::into_bytes::into_bytes;
use std::io::Cursor;

extern crate crypto;

use self::crypto::digest::Digest;
use self::crypto::sha2::Sha256;

mod errors {
    error_chain!{}
}
use errors::*;
//use ::payload::payload::Verack;
//use codec::msgs::msg::payload::payload::Verack;

pub mod commons;
pub mod header;
pub mod payload;

#[derive(Clone)]
pub struct Msg {
    pub header: header::Header,
    pub payload: Option<payload::Payload>,
}

impl Msg {
    pub fn chk<'a, I>(payload_arrvec: I) -> Result<u32>
    where
        I: IntoIterator<Item = &'a u8>,
    {
        let payload_arrvec = payload_arrvec.into_iter();
        let mut sha = [0; 32];
        let mut chk = Sha256::new();
        chk.input(&payload_arrvec.cloned().collect::<Vec<u8>>());
        chk.result(&mut sha);
        chk.reset();
        chk.input(&sha);
        chk.result(&mut sha);

        Cursor::new(&sha).read_u32::<LittleEndian>().chain_err(cf!(
            "Error at u32 parse for payloadchk for value {:?}",
            &sha
        ))
    }
}

impl NewFromHex for Msg {
    fn new<'a, I>(it: I) -> Result<Msg>
    where
        I: IntoIterator<Item = &'a u8>,
    {
        let mut it = it.into_iter();

        let header = header::Header::new(it.by_ref()).chain_err(cf!("Error at creating Header"))?;

        let payload_arrvec = it.cloned().collect::<Vec<u8>>();
        let chk = Msg::chk(payload_arrvec.iter())?;

        let mut it_pl = payload_arrvec.iter();

        if chk != header.payloadchk {
            bail!(ff!(
                "Error at payload checksum (expected: {}, found: {:?})",
                header.payloadchk,
                &chk
            ));
        };

        let payload = match header.cmd {
            header::cmd::Cmd::Tx => {
                let tx = payload::tx::Tx::new(it_pl.by_ref())
                    .chain_err(cf!("Error at creating Payload"))?;
                Some(payload::Payload::Tx(tx))
            }
            header::cmd::Cmd::Ping => {
                let ping =
                    payload::ping::Ping::new(it_pl).chain_err(cf!("Error at creating ping"))?;
                Some(payload::Payload::Ping(ping))
            }
            header::cmd::Cmd::Pong => {
                let pong =
                    payload::pong::Pong::new(it_pl).chain_err(cf!("Error at creating pong"))?;
                Some(payload::Payload::Pong(pong))
            }
            header::cmd::Cmd::Version => {
                let version = payload::version::Version::new(it_pl)
                    .chain_err(cf!("Error at creating version"))?;
                Some(payload::Payload::Version(version))
            }
            header::cmd::Cmd::Verack => Some(payload::Payload::Verack),
            header::cmd::Cmd::SendHeaders => Some(payload::Payload::SendHeaders),
            header::cmd::Cmd::GetHeaders => {
                let get_headers = payload::get_headers::GetHeaders::new(it_pl)
                    .chain_err(cf!("Error at creating get_headers"))?;
                Some(payload::Payload::GetHeaders(get_headers))
            }
            header::cmd::Cmd::Headers => {
                let headers = payload::headers::Headers::new(it_pl)
                    .chain_err(cf!("Error at creating get_headers"))?;
                Some(payload::Payload::Headers(headers))
            }
            header::cmd::Cmd::GetAddr => Some(payload::Payload::GetAddr),
            header::cmd::Cmd::Addr => {
                let addr = payload::addr::Addr::new(it_pl)
                    .chain_err(cf!("Error at creating Addr"))?;
                Some(payload::Payload::Addr(addr))
            }
            header::cmd::Cmd::GetData => {
                let get_data = payload::get_data::GetData::new(it_pl)
                    .chain_err(cf!("Error at creating GetData"))?;
                Some(payload::Payload::GetData(get_data))
            }
            header::cmd::Cmd::Inv => {
                let inv = payload::inv::Inv::new(it_pl)
                    .chain_err(cf!("Error at creating Inv"))?;
                Some(payload::Payload::Inv(inv))
            }
            header::cmd::Cmd::Block => {
                let block = payload::block::Block::new(it_pl)
                    .chain_err(cf!("Error at creating Block"))?;
                Some(payload::Payload::Block(block))
            }
            header::cmd::Cmd::NotFound => {
                let not_found = payload::not_found::NotFound::new(it_pl)
                    .chain_err(cf!("Error at creating NotFound"))?;
                Some(payload::Payload::NotFound(not_found))
            }
        };
        // header.payload_len // TODO
        Ok(Msg { header, payload })
    }
}

impl std::fmt::Debug for Msg {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        let mut s = "Message:\n".to_string();
        s += &format!("├ Message Header: {:?}", self.header);
        s += &"├ Message Payload: \n".to_string();
        s += &match self.payload {
            Some(ref p) => match *p {
                payload::Payload::Tx(ref tx) => format!("{:?}", tx),
                payload::Payload::Ping(ref ping) => format!("{:?}", ping),
                payload::Payload::Pong(ref pong) => format!("{:?}", pong),
                payload::Payload::Version(ref version) => format!("{:?}", version),
                payload::Payload::Verack => "Verack".into(),
                payload::Payload::SendHeaders => "SendHeaders".into(),
                payload::Payload::GetHeaders(ref get_headers) => format!("{:?}", get_headers),
                payload::Payload::Headers(ref headers) => format!("{:?}", headers),
                payload::Payload::GetAddr => "Verack".into(),
                payload::Payload::Addr(ref addr) => format!("{:?}", addr),
                payload::Payload::GetData(ref get_data) => format!("{:?}", get_data),
                payload::Payload::Inv(ref inv) => format!("{:?}", inv),
                payload::Payload::Block(ref block) => format!("{:?}", block),
                payload::Payload::NotFound(ref not_found) => format!("{:?}", not_found),                
            },
            None => "None".to_string(),
        }.lines()
            .map(|x| "│ ".to_string() + x + "\n")
            .collect::<String>();
        write!(f, "{}", s)
    }
}
impl IntoBytes for Msg {
    fn into_bytes(&self) -> Result<Vec<u8>> {
        let mut wrt = vec![];
        wrt.append(&mut self.header.into_bytes()?);
        let mut wrt_payload = match self.clone().payload {
            Some(ref p) => match p {
                &payload::Payload::Tx(ref tx) => tx.into_bytes()?,
                &payload::Payload::Ping(ref ping) => ping.into_bytes()?,
                &payload::Payload::Pong(ref pong) => pong.into_bytes()?,
                &payload::Payload::Version(ref version) => version.into_bytes()?,
                &payload::Payload::Verack => vec![],
                &payload::Payload::SendHeaders => vec![],
                &payload::Payload::GetHeaders(ref get_headers) => get_headers.into_bytes()?,
                &payload::Payload::Headers(ref headers) => headers.into_bytes()?,
                &payload::Payload::GetAddr => vec![],
                &payload::Payload::Addr(ref addr) => addr.into_bytes()?,
                &payload::Payload::Block(ref block) => block.into_bytes()?,
                &payload::Payload::GetData(ref get_data) => get_data.into_bytes()?,
                &payload::Payload::Inv(ref inv) => inv.into_bytes()?,
                &payload::Payload::NotFound(ref not_found) => not_found.into_bytes()?,
            },
            None => vec![],
        };
        wrt.append(&mut wrt_payload);
        Ok(wrt)
    }
}
