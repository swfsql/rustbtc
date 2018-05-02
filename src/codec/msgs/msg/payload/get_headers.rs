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
pub struct GetHeaders {
    pub nonce: u64,
}

impl NewFromHex for GetHeaders {
    fn new(it: &mut std::vec::IntoIter<u8>) -> Result<GetHeaders> {
        unimplemented!("TODO: implement GetHeaders payload NewFromHex")
    }
}

impl std::fmt::Debug for GetHeaders {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        unimplemented!("TODO: implement GetHeaders payload Debug")
    }
}

impl IntoBytes for GetHeaders {
    fn into_bytes(&self) -> Result<Vec<u8>> {
        unimplemented!("TODO: implement GetHeaders payload IntoBytes")
    }
}
