use std;
use std::fmt;
use Commons::NewFromHex::NewFromHex;
use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};
mod errors {
    error_chain!{}
}
use errors::*;

// https://bitcoin.org/en/developer-reference#ping
pub struct Ping {
  pub nounce: u64,
}

impl NewFromHex for Ping {
  fn new(it: &mut std::vec::IntoIter<u8>) -> Result<Ping> {
  //pub fn new(it: &mut std::vec::IntoIter<u8>) -> Result<Box<std::fmt::Debug>, Box<Error>> {

    let aux = it.take(8).collect::<Vec<u8>>();
    let nounce = Cursor::new(&aux)
          .read_u64::<LittleEndian>()
          .chain_err(|| format!("(Msg::Payload::Ping) Failed when n-once tried to read {:?} as u64", aux))?;
    Ok(Ping {
      nounce: nounce,
    })
  }

}

impl std::fmt::Debug for Ping {
  fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
      let mut s = "Ping:\n".to_string();
      s += &format!("├ Nounce: {}\n", self.nounce);
      write!(f, "{}", s)
  }

}

