use arrayvec::ArrayVec;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use codec::msgs::msg::commons::bytes::Bytes;
use codec::msgs::msg::commons::into_bytes::IntoBytes;
use codec::msgs::msg::commons::new_from_hex::NewFromHex;
use std;
use std::fmt;
use codec::msgs::msg::commons::var_uint::VarUint;
use std::io::Cursor;
mod errors {
    error_chain!{}
}
use errors::*;

#[derive(Clone,Debug)]
pub struct BlockHeaders {
    pub version: i32,
    pub prev_block: ArrayVec<[u8; 32]>,
    pub markle_root: ArrayVec<[u8; 32]>,
    pub timestamp: u32,
    pub bits: u32,
    pub nonce: u32,
    pub txn_count: VarUint,
}

impl NewFromHex for BlockHeaders {
    fn new<'a, I>(it: I) -> Result<BlockHeaders>
    where
        I: IntoIterator<Item = &'a u8>,
    {
        let mut it = it.into_iter();
        let aux = it.by_ref().take(4).cloned().collect::<Vec<u8>>();
        let version = Cursor::new(&aux)
            .read_i32::<LittleEndian>()
            .chain_err(cf!("Error read to version as i32 for value {:?}", aux))?;
        //TODO VERSION VERIFY

        let prev_block = it.by_ref()
            .take(32)
            .cloned()
            .collect::<ArrayVec<[u8; 32]>>();

        let markle_root = it.by_ref()
            .take(32)
            .cloned()
            .collect::<ArrayVec<[u8; 32]>>();

        let aux = it.by_ref().take(4).cloned().collect::<Vec<u8>>();
        let timestamp = Cursor::new(&aux)
            .read_u32::<LittleEndian>()
            .chain_err(cf!("Error read to timestamp as i64 for value {:?}", aux))?;

        let aux = it.by_ref().take(4).cloned().collect::<Vec<u8>>();
        let bits = Cursor::new(&aux)
            .read_u32::<LittleEndian>()
            .chain_err(cf!("Error read to bits as i64 for value {:?}", aux))?;

        let aux = it.by_ref().take(4).cloned().collect::<Vec<u8>>();
        let nonce = Cursor::new(&aux)
            .read_u32::<LittleEndian>()
            .chain_err(cf!("Error read to nonce as i64 for value {:?}", aux))?;

        let txn_count = VarUint::new(it.by_ref())
            .chain_err(cf!("Error read to count as new VarUint for length"))?;

        Ok(BlockHeaders {
            version,
            prev_block,
            markle_root,
            timestamp,
            bits,
            nonce,
            txn_count,
        })
    }
}


impl IntoBytes for BlockHeaders {
    fn into_bytes(&self) -> Result<Vec<u8>> {
        let mut wtr = vec![];

        wtr.write_i32::<LittleEndian>(self.version).chain_err(cf!(
            "Failure to convert version ({}) into byte vec",
            self.version
        ))?;
        
        wtr.append(&mut self.prev_block.to_vec());

        wtr.append(&mut self.markle_root.to_vec());

        wtr.write_u32::<LittleEndian>(self.timestamp).chain_err(cf!(
            "Failure to convert timestamp ({}) into byte vec",
            self.timestamp
        ))?;

        wtr.write_u32::<LittleEndian>(self.bits).chain_err(cf!(
            "Failure to convert bits ({}) into byte vec",
            self.bits
        ))?;

        wtr.write_u32::<LittleEndian>(self.nonce).chain_err(cf!(
            "Failure to convert nonce ({}) into byte vec",
            self.nonce
        ))?;

        let mut txn_count = self.txn_count.into_bytes()
            .chain_err(cf!(
                "Failure to convert txn_count ({:?}) into byte vec",
                self.txn_count))?;
        wtr.append(&mut txn_count);

        Ok(wtr)
    }
}