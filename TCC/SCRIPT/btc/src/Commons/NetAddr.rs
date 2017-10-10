use std;
use std::fmt;
use std::error::Error;
use Commons::Bytes::Bytes;
use arrayvec::ArrayVec;
use Commons::NewFromHex::NewFromHex;
use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};

// falta pub time: u32
// https://en.bitcoin.it/wiki/Protocol_documentation#Network_address
pub struct NetAddr {
  pub service: u64,
  pub ip: ArrayVec<[u8; 16]>,
  pub port: u16,
}

impl NewFromHex for NetAddr {
  fn new(it: &mut std::vec::IntoIter<u8>) -> Result<NetAddr, Box<Error>> {
    let service = Cursor::new(it.by_ref().take(8).collect::<Vec<u8>>())
      .read_u64::<LittleEndian>()?;
    let ip = it.by_ref().take(16).map(|u| u.to_le()).collect::<ArrayVec<[u8; 16]>>();
    let port = Cursor::new(it.by_ref().take(2).collect::<Vec<u8>>())
      .read_u16::<LittleEndian>()?;
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


