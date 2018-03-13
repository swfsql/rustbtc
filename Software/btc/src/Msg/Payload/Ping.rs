use std;
use std::fmt;
use commons::new_from_hex::NewFromHex;
use commons::into_bytes::IntoBytes;
use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
mod errors {
    error_chain!{}
}
use errors::*;

// https://bitcoin.org/en/developer-reference#ping
pub struct Ping {
    pub nonce: u64,
}

impl NewFromHex for Ping {
    fn new(it: &mut std::vec::IntoIter<u8>) -> Result<Ping> {
        //pub fn new(it: &mut std::vec::IntoIter<u8>) -> Result<Box<std::fmt::Debug>, Box<Error>> {

        let aux = it.take(8).collect::<Vec<u8>>();
        let nonce = Cursor::new(&aux).read_u64::<LittleEndian>().chain_err(|| {
            format!(
                "(Msg::payload::ping) Failed when n-once tried to read {:?} as u64",
                aux
            )
        })?;
        Ok(Ping { nonce: nonce })
    }
}

impl std::fmt::Debug for Ping {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        let mut s = "Ping:\n".to_string();
        s += &format!("├ Nonce: {}\n", self.nonce);
        write!(f, "{}", s)
    }
}

impl IntoBytes for Ping {
    fn into_bytes(&self) -> Result<Vec<u8>> {
        let mut wtr = vec![];
        wtr.write_u64::<LittleEndian>(self.nonce)
            .chain_err(|| format!("Failure to convert nonce ({}) into byte vec", self.nonce))?;
        Ok(wtr)
    }
}
