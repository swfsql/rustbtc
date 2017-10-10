
// https://bitcoin.org/en/developer-reference#ping
#[derive(Debug)]
enum VarUint {
  U8(u8),
  U16(u16),
  U32(u32),
  U64(u64),
}

impl NewFromHex for VarUint {
  fn new(it: &mut std::vec::IntoIter<u8>) -> Result<VarUint, Box<Error>> {
    let value_head = it.by_ref().next().ok_or("TODO")?.to_le();
    match value_head {
      //0x00 .. 0xFC => VarInt::U8(value_head), // leu 1 byte
      0xFD => {
        let value_body = Cursor::new(it.take(2).collect::<Vec<u8>>())
          .read_u16::<LittleEndian>().unwrap();
        Ok(VarUint::U16(value_body))  // ler 16 bit
      },
      0xFE => { // ler 32 bit
        let value_body = Cursor::new(it.take(4).collect::<Vec<u8>>())
          .read_u32::<LittleEndian>().unwrap();
        Ok(VarUint::U32(value_body))
      },
      0xFF => { // ler 64 bit
        let value_body = Cursor::new(it.take(8).collect::<Vec<u8>>())
          .read_u64::<LittleEndian>().unwrap();
        Ok(VarUint::U64(value_body))
      },
      _ => {

        Ok(VarUint::U8(value_head)) // leu 1 byte
      },

    }
  }
}


