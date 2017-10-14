use std;
use std::fmt;
use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};

use Commons::{NetAddr, VarStr, NewFromHex};
mod errors {
    error_chain!{}
}
use errors::*;

// https://en.bitcoin.it/wiki/Protocol_documentation#version
// https://bitcoin.org/en/developer-reference#version
pub struct Version {
  pub version: i32,
  pub services: u64,
  pub timestamp: i64,
  pub addr_recv: NetAddr::NetAddr,
  pub addr_trans: NetAddr::NetAddr,
  pub nonce: u64,
  pub user_agent: VarStr::VarStr,
  pub start_height: i32,
  pub relay: Option<bool>,
}

// https://bitcoin.org/en/developer-reference#protocol-versions
impl NewFromHex::NewFromHex for Version {
  fn new(it: &mut std::vec::IntoIter<u8>) -> Result<Version> {

    let aux = it.by_ref().take(4).collect::<Vec<u8>>();
    let version = Cursor::new(&aux)
      .read_i32::<LittleEndian>()
      .chain_err(|| format!("Error read to version as i32 for value {:?}", aux))?;
    if version < 60002i32 {
      Err(format!("Unsuported protocol version: <{}>", version))?
    }
    let aux = it.by_ref().take(8).collect::<Vec<u8>>();
    let services = Cursor::new(&aux).read_u64::<LittleEndian>()
      .chain_err(|| format!("Error read to services as i64 for value {:?}", aux))?;
    let aux = it.by_ref().take(8).collect::<Vec<u8>>();
    let timestamp = Cursor::new(&aux).read_i64::<LittleEndian>().
      chain_err(|| format!("Error read to timestamp as i64 for value {:?}", aux))?;
    let addr_recv = NetAddr::NetAddr::new(it)?;
    let addr_trans = NetAddr::NetAddr::new(it)?;
    let aux = it.by_ref().take(8).collect::<Vec<u8>>();
    let nonce = Cursor::new(&aux).read_u64::<LittleEndian>()
      .chain_err(|| format!("Error read to services as read_i64 for value {:?}", aux))?;
    let user_agent = VarStr::VarStr::new(it)?;
    let aux = it.by_ref().take(4).collect::<Vec<u8>>();
    let start_height = Cursor::new(&aux).read_i32::<LittleEndian>()
      .chain_err(|| format!("Error read to services as read_i64 for value {:?}", aux))?;
    let relay = if version < 70002i32 {
      None
    } else {
      let aux = it.by_ref().next()
        .ok_or("Error: input feed ended unexpectdly")?;
      Some(aux.to_le() != 0u8)
    };
    Ok(Version{
      version: version,
      services: services,
      timestamp: timestamp,
      addr_recv: addr_recv,
      addr_trans: addr_trans,
      nonce: nonce,
      user_agent: user_agent,
      start_height: start_height,
      relay: relay,
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
