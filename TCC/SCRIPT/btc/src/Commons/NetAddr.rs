use std;
use std::fmt;
use Commons::Bytes::Bytes;
use arrayvec::ArrayVec;
use Commons::NewFromHex::NewFromHex;
use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};
mod errors {
    error_chain!{}
}
use errors::*;

// falta pub time: u32
// https://en.bitcoin.it/wiki/Protocol_documentation#Network_address
pub struct NetAddr {
  pub service: u64,
  pub ip: ArrayVec<[u8; 16]>,
  pub port: u16,
}

impl NewFromHex for NetAddr {
  fn new(it: &mut std::vec::IntoIter<u8>) -> Result<NetAddr> {
    let aux = it.by_ref().take(8).collect::<Vec<u8>>();
    let service = Cursor::new(&aux).read_u64::<LittleEndian>()
      .chain_err(|| format!("Error at u64 parse for service for value {:?}", aux))?;
    let ip = it.by_ref().take(16).map(|u| u.to_le()).collect::<ArrayVec<[u8; 16]>>();
    let aux = it.by_ref().take(2).collect::<Vec<u8>>();
    let port = Cursor::new(&aux).read_u16::<LittleEndian>()
      .chain_err(|| format!("Error at u16 parse for port for value {:?}", aux))?;
    Ok(NetAddr{
      service: service,
      ip: ip,
      port: port,
    })
  }
}

impl std::fmt::Debug for NetAddr {
  fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {

      let mut s = "Net Addr:\n".to_string();
      s += &format!("├ Service: {}\n", self.service);
      s += &format!("├ IP: {:?}\n", self.ip
        .clone().into_iter().collect::<Bytes>());
      s += &format!("├ Port: {}", self.port);
      write!(f, "{}", s)
  }
}


