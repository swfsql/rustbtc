use codec::msgs::msg::commons::new_from_hex::NewFromHex;
use std::io::Cursor;

mod errors {
    error_chain!{}
}
use errors::*;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use codec::msgs::msg::commons::into_bytes::IntoBytes;

// https://bitcoin.org/en/developer-reference#ping
#[derive(Debug, Clone)]
pub enum VarUint {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
}

impl NewFromHex for VarUint {
    fn new<'a, I>(it: I) -> Result<VarUint>
    where
        I: IntoIterator<Item = &'a u8>,
    {
        let mut it = it.into_iter();
        let value_head = it.next()
            .ok_or(ff!("Error at creating value_head"))?
            .to_le();
        match value_head {
            0x00..0xFC => VarUint::read_u8(&[value_head]), // leu 1 byte
            0xFD => {
                let aux = it.by_ref().take(2).cloned().collect::<Vec<u8>>();
                VarUint::read_u16(&aux)
            }
            0xFE => {
                // ler 32 bit
                let aux = it.by_ref().take(4).cloned().collect::<Vec<u8>>();
                VarUint::read_u32(&aux)
            }
            0xFF => {
                // ler 64 bit
                let aux = it.by_ref().take(8).cloned().collect::<Vec<u8>>();
                VarUint::read_u64(&aux)
            }
            _ => panic!(ff!("Unexpected byte on VarUint")),
        }
    }
}

impl VarUint {
    pub fn from_bytes(bytes: &[u8]) -> VarUint {
        let len = bytes.len();
        match len {
            0x00..0xFC => VarUint::U8(len as u8), // leu 1 byte
            0x00_FD..0xFF_FF => VarUint::U16(len as u16),
            0x00_01_00_00..0xFF_FF_FF_FF => VarUint::U32(len as u32),
            0x00_00_00_01_00_00_00_00..0xFF_FF_FF_FF_FF_FF_FF_FF => VarUint::U64(len as u64),
            _ => panic!(ff!("too many bytes fir a single VarUint")),
        }
    }

    // overkill, but inserted for code consistency
    fn read_u8(bytes: &[u8]) -> Result<VarUint> {
        let value_body = Cursor::new(bytes)
            .read_u8()
            .chain_err(cf!("Failed when VarUint tried to read {:?} as u8", bytes))?;
        Ok(VarUint::U8(value_body))
    }

    fn read_u16(bytes: &[u8]) -> Result<VarUint> {
        let value_body = Cursor::new(bytes)
            .read_u16::<LittleEndian>()
            .chain_err(cf!("Failed when VarUint tried to read {:?} as u16", bytes))?;
        Ok(VarUint::U16(value_body))
    }

    fn read_u32(bytes: &[u8]) -> Result<VarUint> {
        let value_body = Cursor::new(bytes)
            .read_u32::<LittleEndian>()
            .chain_err(cf!("Failed when VarUint tried to read {:?} as u32", bytes))?;
        Ok(VarUint::U32(value_body))
    }

    fn read_u64(bytes: &[u8]) -> Result<VarUint> {
        let value_body = Cursor::new(bytes)
            .read_u64::<LittleEndian>()
            .chain_err(cf!("Failed when VarUint tried to read {:?} as u64", bytes))?;
        Ok(VarUint::U64(value_body))
    }

    pub fn as_usize(&self) -> Option<usize> {
        match *self {
            VarUint::U8(u) => Some(u as usize),
            VarUint::U16(u) => Some(u as usize),
            VarUint::U32(u) => Some(u as usize),
            VarUint::U64(_) => None, // u64 as usize is uncertain on x86 arch
        }
    }

}

impl IntoBytes for VarUint {
    fn into_bytes(&self) -> Result<Vec<u8>> {
        let mut wtr = vec![];
        match *self {
            VarUint::U8(n) => wtr.write_u8(n)
                .chain_err(cf!("Failure to convert U8 ({}) into byte vec", n))?,
            VarUint::U16(n) => {
                wtr.write_u8(0xFC)
                    .chain_err(cf!("Failure to convert U16 ({}) into byte vec", n))?;
                wtr.write_u16::<LittleEndian>(n)
                    .chain_err(cf!("Failure to convert U16 ({}) into byte vec", n))?;
            }
            VarUint::U32(n) => {
                wtr.write_u8(0xFD)
                    .chain_err(cf!("Failure to convert U32 ({}) into byte vec", n))?;
                wtr.write_u32::<LittleEndian>(n)
                    .chain_err(cf!("Failure to convert U32 ({}) into byte vec", n))?;
            }
            VarUint::U64(n) => {
                wtr.write_u8(0xFF)
                    .chain_err(cf!("Failure to convert U64 ({}) into byte vec", n))?;
                wtr.write_u64::<LittleEndian>(n)
                    .chain_err(cf!("Failure to convert U64 ({}) into byte vec", n))?;
            }
        };
        Ok(wtr)
    }
}
