//use arrayvec::ArrayVec;
//use std;
//use std::fmt;
//use std::io::Cursor;

//use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
//use codec::msgs::msg::commons::{net_addr, new_from_hex, var_str};

//use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use codec::msgs::msg::commons::into_bytes::IntoBytes;
use codec::msgs::msg::commons::new_from_hex::NewFromHex;
use codec::msgs::msg::commons::var_uint::VarUint;
use codec::msgs::msg::payload::tx::Tx;
use codec::msgs::msg::commons::block_headers::BlockHeaders;
//use std::io::Cursor;
mod errors {
    error_chain!{}
}
use errors::*;

#[derive(Clone,Debug)]
pub struct Block {
    pub block_header: BlockHeaders,
    pub txn_count: VarUint,
    pub txns: Vec<Tx>,
}

impl NewFromHex for Block {
    fn new<'a, I>(it: I) -> Result<Block>
    where
        I: IntoIterator<Item = &'a u8>,
    {
        let mut it = it.into_iter();

        let block_header = BlockHeaders::new(it.by_ref())
                .chain_err(cf!("Error at creating a new Block Header"))?;

        let txn_count = VarUint::new(it.by_ref())
            .chain_err(cf!("Error at new VarUint for length"))?;

        let txn_count_usize = txn_count
            .as_usize()
            .ok_or(ff!("Error at creating txn_count length: too big"))?;

        let mut txns: Vec<Tx> = vec![];
        for i in 0..txn_count_usize {
            let aux = Tx::new(&mut it)
                .chain_err(cf!("Error at creating a new Tx, at txns {:?}", i))?;
            txns.push(aux);
        }

        Ok(Block {
            block_header,
            txn_count,
            txns,
        })
    }
}

impl IntoBytes for Block {
    fn into_bytes(&self) -> Result<Vec<u8>> {
        let mut wtr = vec![];

        wtr.append(&mut self.block_header.into_bytes()?);

        let mut txn_count_vec = self.txn_count.into_bytes()
            .chain_err(cf!(
                "Failure to convert txn_count ({:?}) into byte vec",
                self.txn_count))?;
        wtr.append(&mut txn_count_vec);
        //.chain_err(cf!("Failure to convert cmd ({}) into byte vec", self.cmd))?;

        let txns = self.txns
            .iter()
            .map(|tx| tx.into_bytes())
            .collect::<Result<Vec<_>>>()?;

        for mut tx in txns {
            wtr.append(&mut tx);
        }

        Ok(wtr)
    }
}
