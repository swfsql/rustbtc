use std;
use std::fmt;
use Commons::NewFromHex::NewFromHex;
use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};
mod errors {
    error_chain!{}
}
use errors::*;

pub struct Pong {
  pub nounce: u64,
}

impl NewFromHex for Pong {
  fn new(it: &mut std::vec::IntoIter<u8>) -> Result<Pong> {
  //pub fn new(it: &mut std::vec::IntoIter<u8>) -> Result<Box<std::fmt::Debug>, Box<Error>> {

    let nounce = Cursor::new(it.take(8).collect::<Vec<u8>>())
          .read_u64::<LittleEndian>().chain_err(|| "")?;
    Ok(Pong {
      nounce: nounce,
    })
  }
}

impl std::fmt::Debug for Pong {

  fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
      let mut s = "Pong:\n".to_string();
      s += &format!("├ Nounce: {}\n", self.nounce);
      write!(f, "{}", s)
  }
}
