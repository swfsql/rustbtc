
// https://en.bitcoin.it/wiki/Protocol_documentation#tx
pub struct MsgHeader {
  pub network: u32,
  pub cmd: ArrayVec<[u8; 12]>,
  pub payload_len: i32,
  pub payloadchk: u32,
}

impl NewFromHex for MsgHeader {
  fn new(it: &mut std::vec::IntoIter<u8>) -> Result<MsgHeader, Box<Error>> {
    Ok(MsgHeader {
      network: Cursor::new(it.take(4).collect::<Vec<u8>>())
        .read_u32::<LittleEndian>().unwrap(),
      cmd: it.take(12).map(|u| u.to_le()).collect::<ArrayVec<[u8; 12]>>(),
      payload_len: Cursor::new(it.take(4).collect::<Vec<u8>>())
        .read_i32::<LittleEndian>().unwrap(),
      payloadchk: Cursor::new(it.take(4).collect::<Vec<u8>>())
        .read_u32::<LittleEndian>().unwrap(),
    })
  }
}

impl std::fmt::Debug for MsgHeader {
  fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
      let mut s = "Message Header:\n".to_string();
      s += &format!("├ Message Network Identification: {}\n", self.network);
      s += &format!("├ Message Command OP_CODE: {:?}\n",
        self.cmd.clone().into_iter().collect::<Bytes>());
      //str::from_utf8
      s += &format!("├ Payload Length: {}\n", self.payload_len);
      s += &format!("├ Payload Checksum: {}\n", self.payloadchk);

      write!(f, "{}", s)
  }
}
