use arrayvec::ArrayVec;
use std;
use std::fmt;
use std::io::Cursor;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use codec::msgs::msg::commons::{net_addr, new_from_hex, var_str};

//use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use codec::msgs::msg::commons::into_bytes::IntoBytes;
use codec::msgs::msg::commons::new_from_hex::NewFromHex;
use codec::msgs::msg::commons::var_uint::VarUint;
//use std::io::Cursor;
mod errors {
    error_chain!{}
}
use errors::*;

pub mod block_headers;

// https://bitcoin.org/en/developer-reference#ping

#[derive(Clone,Debug)]
pub struct Headers {
    pub count: VarUint,
    pub headers: Vec<block_headers::BlockHeaders>,
}

impl NewFromHex for Headers {
    fn new<'a, I>(it: I) -> Result<Headers>
    where
        I: IntoIterator<Item = &'a u8>,
    {

        let mut it = it.into_iter();

        let count = VarUint::new(it.by_ref())
            .chain_err(cf!("Error at to read count as new VarUint for length"))?;
        let count_usize = count
            .as_usize()
            .ok_or(ff!("Error at creating HashCount length: too big"))?;

        let mut headers: Vec<block_headers::BlockHeaders> = vec![];
        for i in 0..count_usize {
            let aux =  block_headers::BlockHeaders::new(it.by_ref())
                .chain_err(cf!("Error at creating a new Output, at outputs {}", i))?;
            headers.push(aux);
        }

        Ok(Headers {
            count,
            headers,
        })
    }
}

impl IntoBytes for Headers {
    fn into_bytes(&self) -> Result<Vec<u8>> {
        let mut wtr = vec![];

        let mut count= self.count.into_bytes()
            .chain_err(cf!(
                "Failure to convert count ({:?}) into byte vec",
                self.count))?;
        wtr.append(&mut count);

        let headers = self.headers
            .iter()
            .map(|headers| headers.into_bytes())
            .collect::<Result<Vec<_>>>()?;
        for mut headers in headers {
            wtr.append(&mut headers);
        }

        Ok(wtr)
    }
}

/*
// TODO:
impl std::fmt::Debug for GetHeaders {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        unimplemented!("TODO: implement GetHeaders payload Debug")
    }
}
*/