use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use codec::msgs::msg::commons::into_bytes::IntoBytes;
use codec::msgs::msg::commons::new_from_hex::NewFromHex;
use std;
use std::fmt;
use std::io::Cursor;

mod errors {
    error_chain!{}
}
use errors::*;

#[derive(Clone)]
pub struct Pong {
    pub nonce: u64,
}

impl NewFromHex for Pong {
    fn new(it: &mut std::vec::IntoIter<u8>) -> Result<Pong> {
        //pub fn new(it: &mut std::vec::IntoIter<u8>) -> Result<Box<std::fmt::Debug>, Box<Error>> {
        let aux = it.take(8).collect::<Vec<u8>>();
        let nonce = Cursor::new(&aux).read_u64::<LittleEndian>().chain_err(|| {
            format!(
                "(Msg::payload::pong) Failed when n-once tried to read {:?} as u64",
                aux
            )
        })?;
        Ok(Pong { nonce })
    }
}

impl std::fmt::Debug for Pong {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        let mut s = "Pong:\n".to_string();
        s += &format!("├ Nounce: {}\n", self.nonce);
        write!(f, "{}", s)
    }
}

impl IntoBytes for Pong {
    fn into_bytes(&self) -> Result<Vec<u8>> {
        let mut wtr = vec![];
        wtr.write_u64::<LittleEndian>(self.nonce)
            .chain_err(|| format!("Failure to convert nonce ({}) into byte vec", self.nonce))?;
        Ok(wtr)
    }
}
