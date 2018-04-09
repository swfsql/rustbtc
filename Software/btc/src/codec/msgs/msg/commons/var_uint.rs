
use std;
use std::io::Cursor;
use codec::msgs::msg::commons::new_from_hex::NewFromHex;
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
            0x00 .. 0xFC => VarUint::read_u8(&[value_head]), // leu 1 byte
            0xFD => {
                let aux = it.take(2).collect::<Vec<u8>>();
                VarUint::read_u16(&aux)
            },
            0xFE => {
                // ler 32 bit
                let aux = it.take(4).collect::<Vec<u8>>();
                VarUint::read_u32(&aux)
            },
            0xFF => {
                // ler 64 bit
                let aux = it.take(8).collect::<Vec<u8>>();
                VarUint::read_u64(&aux)
            },
            _ => panic!("Unexpected byte on VarUint"),
        }
    }
}


impl VarUint {
    pub fn from_bytes(bytes: &[u8]) -> Option<Result<VarUint>> {
        match bytes.len() {
            0 => None,
            1 => Some(VarUint::read_u8(&[bytes[0]])),
            2..3 => Some(VarUint::read_u16(&bytes[1..])),
            4..5 => Some(VarUint::read_u32(&bytes[1..])),
            6..9 => Some(VarUint::read_u64(&bytes[1..])),
            _len => None, // TODO: Some(error), showing the invalid len
        }

    }

    // overkill, but inserted for code consistency
    fn read_u8(bytes: &[u8]) -> Result<VarUint> {
        let value_body = Cursor::new(bytes)
            .read_u8()
            .chain_err(|| {
                format!(
                    "(Commons::var_uint) Failed when VarUint tried to read {:?} as u8",
                    bytes
                )
            })?;
        Ok(VarUint::U8(value_body))
    }

    fn read_u16(bytes: &[u8]) -> Result<VarUint> {
        let value_body = Cursor::new(bytes)
            .read_u16::<LittleEndian>()
            .chain_err(|| {
                format!(
                    "(Commons::var_uint) Failed when VarUint tried to read {:?} as u16",
                    bytes
                )
            })?;
        Ok(VarUint::U16(value_body))
    }

    fn read_u32(bytes: &[u8]) -> Result<VarUint> {
        let value_body = Cursor::new(bytes)
            .read_u32::<LittleEndian>()
            .chain_err(|| {
                format!(
                    "(Commons::var_uint) Failed when VarUint tried to read {:?} as u32",
                    bytes
                )
            })?;
        Ok(VarUint::U32(value_body))
    }

    fn read_u64(bytes: &[u8]) -> Result<VarUint> {
        let value_body = Cursor::new(bytes)
            .read_u64::<LittleEndian>()
            .chain_err(|| {
                format!(
                    "(Commons::var_uint) Failed when VarUint tried to read {:?} as u64",
                    bytes
                )
            })?;
        Ok(VarUint::U64(value_body))
    }


}