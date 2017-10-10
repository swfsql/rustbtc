
pub struct VarStr {
  length: VarUint,
  string: Bytes,
}

impl NewFromHex for VarStr {
  fn new(it: &mut std::vec::IntoIter<u8>) -> Result<VarStr, Box<Error>> {
    let len = VarUint::new(it).unwrap();
    let slen = match len {
      VarUint::U8(u) => Some(u as usize),
      VarUint::U16(u) => Some(u as usize),
      VarUint::U32(u) => Some(u as usize),
      VarUint::U64(_) => None, // u64 as usize is uncertain on x86 arch
    };
    Ok(VarStr {
      length: len,
      string: it.take(slen.unwrap()).map(|u| u.to_le()).collect::<Bytes>(),
    })
  }
}

impl std::fmt::Debug for VarStr {

}
  fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
      let mut s = "Version:\n".to_string();
      s += &format!("├ Length: {:?}\n", self.length);
      s += &format!("├ String: {:?}", self.string);
      write!(f, "{}", s)
  }
