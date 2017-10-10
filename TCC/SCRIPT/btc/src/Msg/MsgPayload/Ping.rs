
// https://bitcoin.org/en/developer-reference#ping
pub struct Ping {
  pub nounce: u64,
}

impl NewFromHex for Ping {
  fn new(it: &mut std::vec::IntoIter<u8>) -> Result<Ping, Box<Error>> {
  //pub fn new(it: &mut std::vec::IntoIter<u8>) -> Result<Box<std::fmt::Debug>, Box<Error>> {

    let nounce = Cursor::new(it.take(8).collect::<Vec<u8>>())
          .read_u64::<LittleEndian>()?;
    Ok(Ping {
      nounce: nounce,
    })
  }

}

impl std::fmt::Debug for Ping {
  fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
      let mut s = "Ping:\n".to_string();
      s += &format!("├ Nounce: {}\n", self.nounce);
      write!(f, "{}", s)
  }

}

