use codec::msgs::msg::commons::bytes::Bytes;
use codec::msgs::msg::commons::new_from_hex::NewFromHex;
use codec::msgs::msg::commons::var_uint::VarUint;
use std;
use std::fmt;
mod errors {
    error_chain!{}
}
use errors::*;

use codec::msgs::msg::commons::into_bytes::IntoBytes;
//use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

#[derive(Clone)]
pub struct VarStr {
    length: VarUint,
    string: Bytes,
}

impl NewFromHex for VarStr {
    fn new<'a, I>(it: I) -> Result<VarStr>
    where
        I: IntoIterator<Item = &'a u8>,
    {
        let mut it = it.into_iter();
        let length = VarUint::new(it.by_ref()).chain_err(cf!("Error at new VarUint for length"))?;
        let slen = length
            .as_usize()
            .ok_or(ff!("Error at creating VarStr length: too big"))?;
        let string = it.by_ref().take(slen).map(|u| u.to_le()).collect::<Bytes>();
        Ok(VarStr { length, string })
    }
}

impl VarStr {
    pub fn from_bytes(bytes: &[u8]) -> Result<VarStr> {
        let length = VarUint::from_bytes(&bytes);
        //.chain_err(cf!("Error when getting a VarStr length"))?;
        let string = Bytes::new(bytes.to_vec());
        Ok(VarStr { length, string })
    }
}

impl std::fmt::Debug for VarStr {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        let mut s = "Version:\n".to_string();
        s += &format!("├ Length: {:?}\n", self.length);
        s += &format!("├ String: {:?}", self.string);
        write!(f, "{}", s)
    }
}

impl IntoBytes for VarStr {
    fn into_bytes(&self) -> Result<Vec<u8>> {
        let mut wtr = vec![];
        wtr.append(&mut self.length.into_bytes().expect(&ff!("Expected length")));
        wtr.append(&mut self.string.into_bytes().expect(&ff!("Expected string")));
        Ok(wtr)
    }
}
