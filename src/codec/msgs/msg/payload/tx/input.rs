use arrayvec::ArrayVec;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use codec::msgs::msg::commons::bytes::Bytes;
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
pub struct Input {
    pub prev_tx: ArrayVec<[u8; 32]>,
    pub prev_tx_out_index: u32,
    pub script_len: u8,
    pub script_sig: Bytes,
    pub sequence: u32,
}

impl NewFromHex for Input {
    fn new<'a, I>(it: I) -> Result<Input>
    where
        I: IntoIterator<Item = &'a u8>,
    {
        let mut it = it.into_iter();
        let prev_tx = it.by_ref().take(32)
            //.map(|u| u.to_le())
            .cloned()
            .collect::<ArrayVec<[u8; 32]>>();
        let aux = it.by_ref().take(4).cloned().collect::<Vec<u8>>();
        let prev_tx_out_index = Cursor::new(&aux).read_u32::<LittleEndian>().chain_err(|| {
            format!("(Msg::payload::tx::input) Error at reading for prev_tx_out_index: read_u32 for value {:?}", aux)
        })?;
        let script_len = it.next()
            .chain_err(|| {
                "Msg::payload::tx::input) Error at reading for slen: Iterator returned unexpected None"
            })?
            .to_le();
        let script_sig = it.by_ref().take(script_len as usize)
            //.map(|u| u.to_le())
            .cloned()
            .collect::<Bytes>();
        let aux = it.by_ref().take(4).cloned().collect::<Vec<u8>>();
        let sequence = Cursor::new(&aux).read_u32::<LittleEndian>().chain_err(|| {
            format!(
                "(Msg::payload::tx::input) Error at u32 for sequence for value {:?}",
                aux
            )
        })?;

        Ok(Input {
            prev_tx,
            prev_tx_out_index,
            script_len,
            script_sig,
            sequence,
        })
    }
}

impl std::fmt::Debug for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        let mut s = "Input:\n".to_string();
        s += &format!(
            "├ Previous Tx: {:?}\n",
            self.prev_tx.clone().into_iter().collect::<Bytes>()
        );
        s += &format!("├ Previous Tx Out Index: {}\n", self.prev_tx_out_index);
        s += &format!("├ Script Length: {}\n", self.script_len);
        s += &format!("├ Script Signature: {:?}\n", self.script_sig);
        s += &format!("├ Sequence: {}\n", self.sequence);

        write!(f, "{}", s)
    }
}

impl IntoBytes for Input {
    fn into_bytes(&self) -> Result<Vec<u8>> {
        let mut wtr = vec![];
        wtr.append(&mut self.prev_tx.to_vec());
        wtr.write_u32::<LittleEndian>(self.prev_tx_out_index)
            .chain_err(|| {
                format!(
                    "Failure to convert prev_tx_out_index ({}) into byte vec",
                    self.prev_tx_out_index
                )
            })?;
        wtr.write_u8(self.script_len).chain_err(|| {
            format!(
                "Failure to convert script_len ({}) into byte vec",
                self.script_len
            )
        })?;
        wtr.append(&mut self.script_sig.into_bytes()?);
        wtr.write_u32::<LittleEndian>(self.sequence).chain_err(|| {
            format!(
                "Failure to convert sequence ({}) into byte vec",
                self.sequence
            )
        })?;
        Ok(wtr)
    }
}
