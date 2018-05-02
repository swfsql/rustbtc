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
pub struct Output {
    pub value: i64,
    pub pk_script_len: u8,
    pub pk_script: Bytes,
}

impl NewFromHex for Output {
    fn new<'a, I>(it: I) -> Result<Output>
    where
        I: IntoIterator<Item = &'a u8>,
    {
        let mut it = it.into_iter();
        let aux = it.by_ref().take(8).cloned().collect::<Vec<u8>>();
        let value = Cursor::new(&aux).read_i64::<LittleEndian>().chain_err(|| {
            format!(
                "(Msg::payload::tx::output) Error at reading for value: read_i64 for {:?}",
                aux
            )
        })?;
        let pk_script_len = it.next()
            .chain_err(|| {
                "(Msg::payload::tx::output) Input unexpectedly ended when reading pk_script_len"
            })?
            .to_le();
        let pk_script = it.by_ref().take(pk_script_len as usize)
            //.map(|u| u.to_le())
            .cloned()
            .collect::<Bytes>();

        Ok(Output {
            value,
            pk_script_len,
            pk_script,
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

impl IntoBytes for Output {
    fn into_bytes(&self) -> Result<Vec<u8>> {
        let mut wtr = vec![];
        wtr.write_i64::<LittleEndian>(self.value)
            .chain_err(|| format!("Failure to convert value ({}) into byte vec", self.value))?;

        wtr.write_u8(self.pk_script_len).chain_err(|| {
            format!(
                "Failure to convert pk_script_len ({}) into byte vec",
                self.pk_script_len
            )
        })?;

        wtr.append(&mut self.pk_script.into_bytes()?);

        Ok(wtr)
    }
}
