use std;
use std::fmt;
use std::error::Error;
use Commons::NewFromHex::NewFromHex;
use Commons::Bytes::Bytes;
use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};

pub struct Output {
  pub value: i64,
  pub pk_script_len: u8,
  pub pk_script: Bytes,
}

impl NewFromHex for Output {
  fn new(it: &mut std::vec::IntoIter<u8>) -> Result<Output, Box<Error>> {
      let val = Cursor::new(it.by_ref().take(8).collect::<Vec<u8>>())
        .read_i64::<LittleEndian>().unwrap();
      let pkslen = it.by_ref().next().unwrap().to_le();

      Ok(Output {
        value: val,
        pk_script_len: pkslen,
        pk_script: it.take(pkslen as usize).map(|u| u.to_le())
          .collect::<Bytes>(),
      })
  }
}

impl std::fmt::Debug for Output {
  fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
      let mut s = "Output:\n".to_string();
      s += &format!("├ Value: {}\n", self.value);
      s += &format!("├ PubKey Script Length: {}\n", self.pk_script_len);
      s += &format!("├ PubKey Script: {:?}\n", self.pk_script);

      write!(f,"{}", s)
  }
}
