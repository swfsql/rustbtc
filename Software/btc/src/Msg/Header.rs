use std;
use std::fmt;
use arrayvec::ArrayVec;
use Commons::NewFromHex::NewFromHex;
use Commons::Bytes::Bytes;
use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};

mod errors {
    error_chain!{}
}
use errors::*;

// https://en.bitcoin.it/wiki/Protocol_documentation#tx
pub struct Header {
  pub network: u32,
  pub cmd: ArrayVec<[u8; 12]>,
  pub payload_len: i32,
  pub payloadchk: u32,
}

impl NewFromHex for Header {
  fn new(it: &mut std::vec::IntoIter<u8>) -> Result<Header> {
    let aux = it.take(4).collect::<Vec<u8>>();
    let network = Cursor::new(&aux).read_u32::<LittleEndian>()
      .chain_err(|| format!("(Msg::Header) Error at u32 parse for network for value {:?}", aux))?;
    let cmd = it.take(12).map(|u| u.to_le())
      .collect::<ArrayVec<[u8; 12]>>();
    let aux = it.take(4).collect::<Vec<u8>>();
    let payload_len = Cursor::new(&aux).read_i32::<LittleEndian>()
        .chain_err(|| format!("(Msg::Header) Error at i32 parse for payload_len for value {:?}", aux))?;
    let aux = it.take(4).collect::<Vec<u8>>();
    let payloadchk = Cursor::new(&aux)
        .read_u32::<LittleEndian>()
        .chain_err(|| format!("(Msg::Header) Error at u32 parse for payloadchk for value {:?}", aux))?;
    Ok(Header {
      network: network,
      cmd: cmd,
      payload_len: payload_len,
      payloadchk: payloadchk,
    })
  }
}

impl std::fmt::Debug for Header {
  fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
      let mut s = "Message Header:\n".to_string();
      s += &format!("├ Message Network Identification: {}\n", self.network);
      s += &format!("├ Message Command OP_CODE: {:?}\n",
        self.cmd.clone().into_iter().collect::<Bytes>());
      //str::from_utf8
      s += &format!("├ Payload Length: {}\n", self.payload_len);
      s += &format!("├ Payload Checksum: {}\n", self.payloadchk);

      write!(f, "{}", s)
  }
}
