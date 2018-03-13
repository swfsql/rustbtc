use std;
use std::fmt;
use arrayvec::ArrayVec;
use commons::bytes::Bytes;
use commons::new_from_hex::NewFromHex;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;
mod errors {
    error_chain!{}
}
use errors::*;

pub struct Input {
    pub prev_tx: ArrayVec<[u8; 32]>,
    pub prev_tx_out_index: u32,
    pub script_len: u8,
    pub script_sig: Bytes,
    pub sequence: u32,
}

impl NewFromHex for Input {
    fn new(it: &mut std::vec::IntoIter<u8>) -> Result<Input> {
        let ptx = it.take(32)
            .map(|u| u.to_le())
            .collect::<ArrayVec<[u8; 32]>>();
        let aux = it.take(4).collect::<Vec<u8>>();
        let ptxoi = Cursor::new(&aux).read_u32::<LittleEndian>().chain_err(|| {
            format!("(Msg::payload::tx::input) Error at reading for prev_tx_out_index: read_u32 for value {:?}", aux)
        })?;
        let slen = it.by_ref()
            .next()
            .chain_err(|| {
                "Msg::payload::tx::input) Error at reading for slen: Iterator returned unexpected None"
            })?
            .to_le();
        let script_sig = it.take(slen as usize).map(|u| u.to_le()).collect::<Bytes>();
        let aux = it.take(4).collect::<Vec<u8>>();
        let sequence = Cursor::new(&aux).read_u32::<LittleEndian>().chain_err(|| {
            format!(
                "(Msg::payload::tx::input) Error at u32 for sequence for value {:?}",
                aux
            )
        })?;

        Ok(Input {
            prev_tx: ptx,
            prev_tx_out_index: ptxoi,
            script_len: slen,
            script_sig: script_sig,
            sequence: sequence,
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
