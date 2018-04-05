use std;
use std::io::Cursor;
use codec::msg::commons::new_from_hex::NewFromHex;
use byteorder::{LittleEndian, ReadBytesExt};
mod errors {
    error_chain!{}
}
use errors::*;

// https://bitcoin.org/en/developer-reference#ping
#[derive(Debug,Clone)]
pub enum VarUint {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
}

impl NewFromHex for VarUint {
    fn new(it: &mut std::vec::IntoIter<u8>) -> Result<VarUint> {
        let value_head = it.by_ref()
            .next()
            .ok_or("Erro at creating value_head")?
            .to_le();
        match value_head {
            //0x00 .. 0xFC => VarInt::U8(value_head), // leu 1 byte
            0xFD => {
                let aux = it.take(2).collect::<Vec<u8>>();
                let value_body = Cursor::new(&aux).read_u16::<LittleEndian>().chain_err(|| {
                    format!(
                        "(Commons::var_uint) Failed when VarUint tried to read {:?} as u16",
                        aux
                    )
                })?;
                Ok(VarUint::U16(value_body)) // ler 16 bit
            }
            0xFE => {
                // ler 32 bit
                let aux = it.take(4).collect::<Vec<u8>>();
                let value_body = Cursor::new(&aux).read_u32::<LittleEndian>().chain_err(|| {
                    format!(
                        "Commons::var_uint) Failed when VarUint tried to read {:?} as u32",
                        aux
                    )
                })?;
                Ok(VarUint::U32(value_body))
            }
            0xFF => {
                // ler 64 bit
                let aux = it.take(8).collect::<Vec<u8>>();
                let value_body = Cursor::new(&aux).read_u64::<LittleEndian>().chain_err(|| {
                    format!(
                        "Commons::var_uint) Failed when VarUint tried to read {:?} as u64",
                        aux
                    )
                })?;
                Ok(VarUint::U64(value_body))
            }
            _ => {
                Ok(VarUint::U8(value_head)) // leu 1 byte
            }
        }
    }
}
