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

#[derive(Clone)]
pub struct FeeFilter {
    pub fee_rate: u64,
}

impl NewFromHex for FeeFilter {
    fn new<'a, I>(it: I) -> Result<FeeFilter>
    where
        I: IntoIterator<Item = &'a u8>,
    {
        let mut it = it.into_iter();
        //pub fn new<'a, I>(it: I) -> Result<Box<std::fmt::Debug>, Box<Error>>
        let aux = it.by_ref().take(8).cloned().collect::<Vec<u8>>();
        let fee_rate = Cursor::new(&aux)
            .read_u64::<LittleEndian>()
            .chain_err(cf!("Failed when n-once tried to read {:?} as u64", aux))?;
        Ok(FeeFilter { fee_rate })
    }
}

impl std::fmt::Debug for FeeFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        let mut s = "FeeFilter:\n".to_string();
        s += &format!("├ Fee Rate: {}\n", self.fee_rate);
        write!(f, "{}", s)
    }
}

impl IntoBytes for FeeFilter {
    fn into_bytes(&self) -> Result<Vec<u8>> {
        let mut wtr = vec![];
        wtr.write_u64::<LittleEndian>(self.fee_rate).chain_err(cf!(
            "Failure to convert fee_rate ({}) into byte vec",
            self.fee_rate
        ))?;
        Ok(wtr)
    }
}
