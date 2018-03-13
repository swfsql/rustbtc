use std;
use std::fmt;
use commons::new_from_hex::NewFromHex;
use commons::bytes::Bytes;
use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};
mod errors {
    error_chain!{}
}
use errors::*;

pub struct Output {
    pub value: i64,
    pub pk_script_len: u8,
    pub pk_script: Bytes,
}

impl NewFromHex for Output {
    fn new(it: &mut std::vec::IntoIter<u8>) -> Result<Output> {
        let aux = it.by_ref().take(8).collect::<Vec<u8>>();
        let val = Cursor::new(&aux).read_i64::<LittleEndian>().chain_err(|| {
            format!(
                "(Msg::payload::tx::output) Error at reading for val: read_i64 for {:?}",
                aux
            )
        })?;
        let pkslen = it.by_ref()
            .next()
            .chain_err(|| "(Msg::payload::tx::output) Input unexpectedly ended when reading pkslen")?
            .to_le();
        let pk_script = it.take(pkslen as usize)
            .map(|u| u.to_le())
            .collect::<Bytes>();

        Ok(Output {
            value: val,
            pk_script_len: pkslen,
            pk_script: pk_script,
        })
    }
}

impl std::fmt::Debug for Output {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        let mut s = "Output:\n".to_string();
        s += &format!("├ Value: {}\n", self.value);
        s += &format!("├ PubKey Script Length: {}\n", self.pk_script_len);
        s += &format!("├ PubKey Script: {:?}\n", self.pk_script);

        write!(f, "{}", s)
    }
}
