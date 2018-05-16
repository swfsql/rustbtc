use arrayvec::ArrayVec;
//use std;
//use std::fmt;
use std::io::Cursor;

use byteorder::{ ReadBytesExt, WriteBytesExt};
//use codec::msgs::msg::commons::{net_addr, new_from_hex, var_str};

//use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use codec::msgs::msg::commons::into_bytes::IntoBytes;
use codec::msgs::msg::commons::new_from_hex::NewFromHex;
use codec::msgs::msg::commons::var_str::VarStr;
//use std::io::Cursor;
mod errors {
    error_chain!{}
}
use errors::*;

// https://bitcoin.org/en/developer-reference#ping

#[derive(Clone, Debug)]
pub struct Reject {
    pub message: VarStr,
    pub code: u8,
    pub reason: VarStr,
    pub extra_data: Option<ArrayVec<[u8; 32]>>,
}

impl NewFromHex for Reject {
    fn new<'a, I>(it: I) -> Result<Reject>
    where
        I: IntoIterator<Item = &'a u8>,
    {
        let mut it = it.into_iter();

        let message = VarStr::new(it.by_ref())?;    

        let aux = it.by_ref().take(1).cloned().collect::<Vec<u8>>();
        let code = Cursor::new(&aux)
            .read_u8()
            .chain_err(cf!("Error read to code as i32 for value {:?}", aux))?;
        
        let reason = VarStr::new(it.by_ref())?;    

        let extra_data;
        if code == 0x10 || code ==0x11 || code ==0x12 || code ==0x40 || code ==0x41 || code ==0x42 ||code == 0x43 {
            extra_data = Some(it.by_ref()
                .take(32)
                .cloned()
                .collect::<ArrayVec<[u8; 32]>>()
            );
        }
        else{
            extra_data = None;
        }

        Ok(Reject {
            message,
            code,
            reason,
            extra_data,
        })
    }
}

impl IntoBytes for Reject {
    fn into_bytes(&self) -> Result<Vec<u8>> {
        let mut wtr = vec![];

        wtr.append(&mut self.message.into_bytes()?);

        wtr.write_u8(self.code).chain_err(cf!(
            "Failure to convert code number ({:?}) into byte vec",
            self.code
        ))?;

        wtr.append(&mut self.reason.into_bytes()?);

        if let Some(ref aux) = self.extra_data{
            wtr.append(&mut aux.to_vec());
        }              

        Ok(wtr)
    }
}
