use std;
use std::fmt;
use commons::bytes::Bytes;
use commons::new_from_hex::NewFromHex;
use commons::var_uint::VarUint;
mod errors {
    error_chain!{}
}
use errors::*;

pub struct VarStr {
    length: VarUint,
    string: Bytes,
}

impl NewFromHex for VarStr {
    fn new(it: &mut std::vec::IntoIter<u8>) -> Result<VarStr> {
        let length = VarUint::new(it).chain_err(|| "Error at new VarUint for length")?;
        let slen = match length {
            VarUint::U8(u) => Some(u as usize),
            VarUint::U16(u) => Some(u as usize),
            VarUint::U32(u) => Some(u as usize),
            VarUint::U64(_) => None, // u64 as usize is uncertain on x86 arch
        };
        let slen = slen.ok_or("(Commons::var_str) Error at creating VarStr length: too big")?;
        let string = it.take(slen).map(|u| u.to_le()).collect::<Bytes>();
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
