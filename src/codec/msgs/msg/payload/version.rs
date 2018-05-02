use std;
use std::fmt;
use std::io::Cursor;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use codec::msgs::msg::commons::into_bytes::IntoBytes;
use codec::msgs::msg::commons::{net_addr, new_from_hex, var_str};
mod errors {
    error_chain!{}
}
use errors::*;

// https://en.bitcoin.it/wiki/Protocol_documentation#version
// https://bitcoin.org/en/developer-reference#version

#[derive(Clone)]
pub struct Version {
    pub version: i32,
    pub services: u64,
    pub timestamp: i64,
    pub addr_recv: net_addr::NetAddr,
    pub addr_trans: net_addr::NetAddr,
    pub nonce: u64,
    pub user_agent: var_str::VarStr,
    pub start_height: i32,
    pub relay: Option<bool>,
}

// https://bitcoin.org/en/developer-reference#protocol-versions
impl new_from_hex::NewFromHex for Version {
    fn new<'a, I>(it: I) -> Result<Version>
    where
        I: IntoIterator<Item = &'a u8>,
    {
        let mut it = it.into_iter();
        let aux = it.by_ref().take(4).cloned().collect::<Vec<u8>>();
        let version = Cursor::new(&aux).read_i32::<LittleEndian>().chain_err(|| {
            format!(
                "(Msg::payload::version) Error read to version as i32 for value {:?}",
                aux
            )
        })?;
        if version < 60_002i32 {
            Err(format!("Unsuported protocol version: <{}>", version))?
        }
        let aux = it.by_ref().take(8).cloned().collect::<Vec<u8>>();
        let services = Cursor::new(&aux).read_u64::<LittleEndian>().chain_err(|| {
            format!(
                "(Msg::payload::version) Error read to services as i64 for value {:?}",
                aux
            )
        })?;
        let aux = it.by_ref().take(8).cloned().collect::<Vec<u8>>();
        let timestamp = Cursor::new(&aux).read_i64::<LittleEndian>().chain_err(|| {
            format!(
                "(Msg::payload::version) Error read to timestamp as i64 for value {:?}",
                aux
            )
        })?;
        let addr_recv = net_addr::NetAddr::new(it.by_ref())?;
        let addr_trans = net_addr::NetAddr::new(it.by_ref())?;
        let aux = it.by_ref().take(8).cloned().collect::<Vec<u8>>();
        let nonce = Cursor::new(&aux).read_u64::<LittleEndian>().chain_err(|| {
            format!(
                "(Msg::payload::version) Error read to services as read_i64 for value {:?}",
                aux
            )
        })?;
        let user_agent = var_str::VarStr::new(it.by_ref())?;
        let aux = it.by_ref().take(4).cloned().collect::<Vec<u8>>();
        let start_height = Cursor::new(&aux).read_i32::<LittleEndian>().chain_err(|| {
            format!(
                "(Msg::payload::version) Error read to services as read_i64 for value {:?}",
                aux
            )
        })?;
        let relay = if version < 70_002_i32 {
            None
        } else {
            let aux = it.next()
                .ok_or("(Msg::payload::version) Error: input feed ended unexpectdly for relay")?;
            Some(aux.to_le() != 0u8)
        };
        Ok(Version {
            version,
            services,
            timestamp,
            addr_recv,
            addr_trans,
            nonce,
            user_agent,
            start_height,
            relay,
        })
    }
}

impl std::fmt::Debug for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        let mut s = "Version:\n".to_string();
        s += &format!("├ Version: {}\n", self.version);
        s += &format!("├ Services: {}\n", self.services);
        s += &format!("├ Timestamp: {}\n", self.timestamp);
        s += &format!("├ Addr Receiver: {:?}\n", self.addr_recv);
        s += &format!("├ Addr Transfer: {:?}\n", self.addr_trans);
        s += &format!("├ Nonce: {}\n", self.nonce);
        s += &format!("├ User Agent: {:?}\n", self.user_agent);
        s += &format!("├ Start Height: {}\n", self.start_height);
        s += &format!("├ Relay: {:?}\n", self.relay);
        write!(f, "{}", s)
    }
}

impl IntoBytes for Version {
    fn into_bytes(&self) -> Result<Vec<u8>> {
        let mut wtr = vec![];
        wtr.write_i32::<LittleEndian>(self.version).chain_err(|| {
            format!(
                "Failure to convert version ({}) into byte vec",
                self.version
            )
        })?;

        wtr.write_u64::<LittleEndian>(self.services).chain_err(|| {
            format!(
                "Failure to convert services ({}) into byte vec",
                self.services
            )
        })?;
        wtr.write_i64::<LittleEndian>(self.timestamp)
            .chain_err(|| {
                format!(
                    "Failure to convert timestamp ({}) into byte vec",
                    self.timestamp
                )
            })?;
        wtr.append(&mut self.addr_recv.into_bytes()?);
        wtr.append(&mut self.addr_trans.into_bytes()?);

        wtr.write_u64::<LittleEndian>(self.nonce)
            .chain_err(|| format!("Failure to convert nonce ({}) into byte vec", self.nonce))?;
        wtr.append(&mut self.user_agent.into_bytes()?);
        wtr.write_i32::<LittleEndian>(self.start_height)
            .chain_err(|| {
                format!(
                    "Failure to convert start_height ({}) into byte vec",
                    self.start_height
                )
            })?;

        if let Some(relay) = self.relay {
            wtr.write_u8(relay as u8)
                .chain_err(|| format!("Failure to convert relay ({}) into byte vec", self.nonce))?;
        }

        Ok(wtr)
    }
}
