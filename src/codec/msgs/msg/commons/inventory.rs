use arrayvec::ArrayVec;
//use std;
//use std::fmt;
use std::io::Cursor;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
//use codec::msgs::msg::commons::{net_addr, new_from_hex, var_str};

//use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use codec::msgs::msg::commons::into_bytes::IntoBytes;
use codec::msgs::msg::commons::new_from_hex::NewFromHex;
//use codec::msgs::msg::commons::var_uint::VarUint;
//use std::io::Cursor;
mod errors {
    error_chain!{}
}
use errors::*;

// https://bitcoin.org/en/developer-reference#ping
/*
type identifier	uint32_t	The type of object which was hashed. See list of type identifiers below.
32	hash	char[32]
*/
#[derive(Clone, Debug)]
pub struct Inventory {
    ///Each number of this variable represents a unique message. (https://bitcoin.org/en/developer-reference#data-messages)
    pub type_identifier: u32,
    pub hash: ArrayVec<[u8; 32]>,
}

impl NewFromHex for Inventory {
    fn new<'a, I>(it: I) -> Result<Inventory>
    where
        I: IntoIterator<Item = &'a u8>,
    {
        let mut it = it.into_iter();

        let aux = it.by_ref().take(4).cloned().collect::<Vec<u8>>();
        let type_identifier = Cursor::new(&aux).read_u32::<LittleEndian>().chain_err(cf!(
            "Error read to type_services as u32 for value {:?}",
            aux
        ))?;

        let hash = it.by_ref()
            .take(32)
            .cloned()
            .collect::<ArrayVec<[u8; 32]>>();

        Ok(Inventory {
            type_identifier,
            hash,
        })
    }
}

impl IntoBytes for Inventory {
    fn into_bytes(&self) -> Result<Vec<u8>> {
        let mut wtr = vec![];

        wtr.write_u32::<LittleEndian>(self.type_identifier)
            .chain_err(cf!(
                "Failure to convert type_identifier number ({:?}) into byte vec",
                self.type_identifier
            ))?;

        wtr.append(&mut self.hash.to_vec());

        Ok(wtr)
    }
}
