pub mod commons;
pub mod header;
pub mod payload;

use byteorder::{LittleEndian, ReadBytesExt};
use codec::msgs::msg::commons::into_bytes::IntoBytes;
use codec::msgs::msg::commons::new_from_hex::NewFromHex;

use codec::msgs::msg::commons::net_addr::NetAddr;
use codec::msgs::msg::commons::var_str::VarStr;
use std;
use std::fmt;
//use codec::msgs::msg::commons::params::Network;
use codec::msgs::msg::payload::version::Version;
use codec::msgs::msg::payload::Payload;
use arrayvec::ArrayVec;
use codec::msgs::msg::commons::var_uint::VarUint;
use chrono::Utc;
use rand;
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
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
    pub fn new_verack() -> Msg {
        let verack_pl_raw = [];
        let verack_header = header::Header {
            network: header::network::Network::Main,
            cmd: header::cmd::Cmd::Verack,
            payload_len: verack_pl_raw.len() as i32,
            payloadchk: Msg::chk(&verack_pl_raw[..]).expect(&ff!()),
        };

        Msg {
            header: verack_header,
            payload: None,
        }
    }

    //TODO: Add parameters on get_headers
    pub fn new_get_headers() -> Msg {

        let version = 70013_i32;
        let hash_count = VarUint::from_bytes(&[1u8]);
        let mut hash_genesis: ArrayVec<[u8; 32]> = ArrayVec::new();
        (0..32).for_each(|_| hash_genesis.push(0));
        hash_genesis.clone_from_slice(&
            [0x00, 0x00, 0x00, 0x00,  0x00, 0x19, 0xd6, 0x68,
            0x9c, 0x08, 0x5a, 0xe1,  0x65, 0x83, 0x1e, 0x93,
            0x4f, 0xf7, 0x63, 0xae,  0x46, 0xa2, 0xa6, 0xc1,
            0x72, 0xb3, 0xf1, 0xb6,  0x0a, 0x8c, 0xe2, 0x6f]);
        let block_locator_hashes: Vec<ArrayVec<[u8; 32]>> = vec![hash_genesis];
        let mut hash_stop: ArrayVec<[u8; 32]> = ArrayVec::new();
        (0..32).for_each(|_| hash_stop.push(0));
        // hash_stop.clone_from_slice(&[0; 32]);

        let payload = payload::get_headers::GetHeaders {
            version,
            hash_count,
            block_locator_hashes,
            hash_stop,
        };

        let get_headers_pl_raw = payload.into_bytes().expect(&ff!());

        let get_headers_header = header::Header {
            network: header::network::Network::Main,
            cmd: header::cmd::Cmd::GetHeaders,
            payload_len: get_headers_pl_raw.len() as i32,
            payloadchk: Msg::chk(&get_headers_pl_raw[..]).expect(&ff!()),
        };  


        Msg {
            header: get_headers_header,
            payload: Some(Payload::GetHeaders(payload)),
        }
    }

    pub fn new_version(addr: SocketAddr) -> Msg {
        let self_addr = SocketAddr::new(
            IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0x7f00, 1)),
            8333,
        ); // TODO get from toolbox
        let version = 70013_i32;
        let addr_trans = NetAddr::from_socket_addr(&addr);
        let addr_recv = NetAddr::from_socket_addr(&self_addr);
        let services = addr_trans.service;
        let nonce = rand::random::<u64>(); // TODO record into peer and toolbox
        let timestamp = Utc::now().timestamp();
        let start_height = 0_i32; // maybe 1
        let relay = Some(false);
        let agent_bytes = b"/Rustbtc:0.0.1/";
        let user_agent = VarStr::from_bytes(agent_bytes).expect(&ff!());

        d!("version payload creating");
        let version_pl = Version {
            version,
            services,
            timestamp,
            addr_recv,
            addr_trans,
            nonce,
            user_agent,
            start_height,
            relay,
        };
        d!("version payload created");

        let version_pl_raw = version_pl.into_bytes().expect(&ff!());

        let version_header = header::Header {
            network: header::network::Network::Main,
            cmd: header::cmd::Cmd::Version,
            payload_len: version_pl_raw.len() as i32,
            payloadchk: Msg::chk(&version_pl_raw[..]).expect(&ff!()),
        };
        d!("version header created");

        Msg {
            header: version_header,
            payload: Some(Payload::Version(version_pl)),
        }
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
                let addr =
                    payload::addr::Addr::new(it_pl).chain_err(cf!("Error at creating Addr"))?;
                Some(payload::Payload::Addr(addr))
            }
            header::cmd::Cmd::GetData => {
                let get_data = payload::get_data::GetData::new(it_pl)
                    .chain_err(cf!("Error at creating GetData"))?;
                Some(payload::Payload::GetData(get_data))
            }
            header::cmd::Cmd::Inv => {
                let inv = payload::inv::Inv::new(it_pl).chain_err(cf!("Error at creating Inv"))?;
                Some(payload::Payload::Inv(inv))
            }
            header::cmd::Cmd::Block => {
                let block =
                    payload::block::Block::new(it_pl).chain_err(cf!("Error at creating Block"))?;
                Some(payload::Payload::Block(block))
            }
            header::cmd::Cmd::NotFound => {
                let not_found = payload::not_found::NotFound::new(it_pl)
                    .chain_err(cf!("Error at creating NotFound"))?;
                Some(payload::Payload::NotFound(not_found))
            }
            header::cmd::Cmd::FeeFilter => {
                let fee_filter = payload::fee_filter::FeeFilter::new(it_pl)
                    .chain_err(cf!("Error at creating FeeFilter"))?;
                Some(payload::Payload::FeeFilter(fee_filter))
            }
            header::cmd::Cmd::GetBlocks => {
                let get_blocks = payload::get_blocks::GetBlocks::new(it_pl)
                    .chain_err(cf!("Error at creating GetBlocks"))?;
                Some(payload::Payload::GetBlocks(get_blocks))
            }
            header::cmd::Cmd::MemPool => Some(payload::Payload::MemPool),
            header::cmd::Cmd::Reject => {
                let reject = payload::reject::Reject::new(it_pl)
                    .chain_err(cf!("Error at creating Reject"))?;
                Some(payload::Payload::Reject(reject))
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
                payload::Payload::GetBlocks(ref get_blocks) => format!("{:?}", get_blocks),
                payload::Payload::Block(ref block) => format!("{:?}", block),
                payload::Payload::NotFound(ref not_found) => format!("{:?}", not_found),
                payload::Payload::FeeFilter(ref fee_filter) => format!("{:?}", fee_filter),
                payload::Payload::MemPool => "MemPool".into(),
                payload::Payload::Reject(ref reject) => format!("{:?}", reject),
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
                &payload::Payload::GetBlocks(ref get_block) => get_block.into_bytes()?,
                &payload::Payload::Block(ref block) => block.into_bytes()?,
                &payload::Payload::GetData(ref get_data) => get_data.into_bytes()?,
                &payload::Payload::Inv(ref inv) => inv.into_bytes()?,
                &payload::Payload::NotFound(ref not_found) => not_found.into_bytes()?,
                &payload::Payload::FeeFilter(ref fee_filter) => fee_filter.into_bytes()?,
                &payload::Payload::MemPool => vec![],
                &payload::Payload::Reject(ref reject) => reject.into_bytes()?,
            },
            None => vec![],
        };
        wrt.append(&mut wrt_payload);
        Ok(wrt)
    }
}
