use arrayvec::ArrayVec;
//use std;
//use std::fmt;
use std::io::Cursor;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
//use codec::msgs::msg::commons::{net_addr, new_from_hex, var_str};

//use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use codec::msgs::msg::commons::into_bytes::IntoBytes;
use codec::msgs::msg::commons::new_from_hex::NewFromHex;
use codec::msgs::msg::commons::var_uint::VarUint;
//use std::io::Cursor;
mod errors {
    error_chain!{}
}
use errors::*;

// https://bitcoin.org/en/developer-reference#ping

#[derive(Clone, Debug)]
pub struct GetBlocks {
    pub version: i32,
    pub hash_count: VarUint,
    pub block_locator_hashes: Vec<ArrayVec<[u8; 32]>>,
    pub hash_stop: ArrayVec<[u8; 32]>,
}

impl NewFromHex for GetBlocks {
    fn new<'a, I>(it: I) -> Result<GetBlocks>
    where
        I: IntoIterator<Item = &'a u8>,
    {
        let mut it = it.into_iter();
        let aux = it.by_ref().take(4).cloned().collect::<Vec<u8>>();
        let version = Cursor::new(&aux)
            .read_i32::<LittleEndian>()
            .chain_err(cf!("Error read to version as i32 for value {:?}", aux))?;

        let hash_count =
            VarUint::new(it.by_ref()).chain_err(cf!("Error at new VarUint for length"))?;
        let hash_count_usize = hash_count
            .as_usize()
            .ok_or(ff!("Error at creating HashCount length: too big"))?;

        let block_locator_hashes = (0..hash_count_usize)
            .map(|_i| {
                it.by_ref()
                    .take(32)
                    .cloned()
                    .collect::<ArrayVec<[u8; 32]>>()
            })
            .fold(vec![], |mut acc, block_loc| {
                acc.push(block_loc);
                acc
            });

        let hash_stop = it.by_ref()
            .take(32)
            .cloned()
            .collect::<ArrayVec<[u8; 32]>>();

        Ok(GetBlocks {
            version,
            hash_count,
            block_locator_hashes,
            hash_stop,
        })
    }
}

impl IntoBytes for GetBlocks {
    fn into_bytes(&self) -> Result<Vec<u8>> {
        let mut wtr = vec![];

        wtr.write_i32::<LittleEndian>(self.version).chain_err(cf!(
            "Failure to convert version number ({:?}) into byte vec",
            self.version
        ))?;

        let mut hash_count_vec = self.hash_count.into_bytes().chain_err(cf!(
            "Failure to convert hash_count ({:?}) into byte vec",
            self.hash_count
        ))?;
        wtr.append(&mut hash_count_vec);
        //.chain_err(cf!("Failure to convert cmd ({}) into byte vec", self.cmd))?;

        self.block_locator_hashes
            .to_vec()
            .iter()
            .for_each(|ref block_loc| {
                wtr.append(&mut block_loc.to_vec());
            });

        wtr.append(&mut self.hash_stop.to_vec());

        Ok(wtr)
    }
}
