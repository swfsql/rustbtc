use std;
use std::fmt;
use Commons::Bytes::Bytes;
use Commons::NewFromHex::NewFromHex;
use Commons::VarUint::VarUint;
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
    let len = VarUint::new(it)
      .chain_err(|| "Error at new VarUint for len")?;
    let slen = match len {
      VarUint::U8(u) => Some(u as usize),
      VarUint::U16(u) => Some(u as usize),
      VarUint::U32(u) => Some(u as usize),
      VarUint::U64(_) => None, // u64 as usize is uncertain on x86 arch
    };
    let slen = slen.ok_or("(Commons::VarStr) Error at creating VarStr length: too big")?;
    let string = it.take(slen).map(|u| u.to_le()).collect::<Bytes>();
    Ok(VarStr {
      length: len,
      string: string,
    })
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


