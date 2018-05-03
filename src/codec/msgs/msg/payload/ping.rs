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

// https://bitcoin.org/en/developer-reference#ping

#[derive(Clone)]
pub struct Ping {
    pub nonce: u64,
}

impl NewFromHex for Ping {
    fn new<'a, I>(it: I) -> Result<Ping>
    where
        I: IntoIterator<Item = &'a u8>,
    {
        //pub fn new<'a, I>(it: I) -> Result<Box<std::fmt::Debug>, Box<Error>>
        let mut it = it.into_iter();

        let aux = it.by_ref().take(8).cloned().collect::<Vec<u8>>();
        let nonce = Cursor::new(&aux)
            .read_u64::<LittleEndian>()
            .chain_err(cf!("Failed when n-once tried to read {:?} as u64", aux))?;
        Ok(Ping { nonce })
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
        wtr.write_u64::<LittleEndian>(self.nonce).chain_err(cf!(
            "Failure to convert nonce ({}) into byte vec",
            self.nonce
        ))?;
        Ok(wtr)
    }
}
