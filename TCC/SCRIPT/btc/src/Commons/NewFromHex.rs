

pub trait NewFromHex {
  fn new_from_hex(hex: &str) -> Result<Self, Box<Error>>
  where Self: std::marker::Sized {
    let vec: Vec<u8> = Vec::from_hex(hex)?;
    let mut it = vec.into_iter();
    Self::new(it.by_ref())
  }
  fn new(it: &mut std::vec::IntoIter<u8>) -> Result<Self, Box<Error>>
  where Self: std::marker::Sized;
}

