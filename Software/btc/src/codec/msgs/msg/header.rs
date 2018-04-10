use std;
use std::fmt;
use arrayvec::ArrayVec;
use codec::msgs::msg::commons::new_from_hex::NewFromHex;
use codec::msgs::msg::commons::bytes::Bytes;
use codec::msgs::msg::commons::into_bytes::IntoBytes;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;


mod errors {
    error_chain!{}
}
use errors::*;

// https://en.bitcoin.it/wiki/Protocol_documentation#tx
#[derive(Clone)]
pub struct Header {
    pub network: u32,
    pub cmd: ArrayVec<[u8; 12]>,
    pub payload_len: i32,
    pub payloadchk: u32,
}

impl NewFromHex for Header {
    fn new(it: &mut std::vec::IntoIter<u8>) -> Result<Header> {
        let aux = it.take(4).collect::<Vec<u8>>();
        let network = Cursor::new(&aux).read_u32::<LittleEndian>().chain_err(|| {
            format!(
                "(Msg::header) Error at u32 parse for network for value {:?}",
                aux
            )
        })?;
        let cmd = it.take(12)
            //.map(|u| u.to_le())
            .collect::<ArrayVec<[u8; 12]>>();
        let aux = it.take(4).collect::<Vec<u8>>();
        let payload_len = Cursor::new(&aux).read_i32::<LittleEndian>().chain_err(|| {
            format!(
                "(Msg::header) Error at i32 parse for payload_len for value {:?}",
                aux
            )
        })?;
        let aux = it.take(4).collect::<Vec<u8>>();
        let payloadchk = Cursor::new(&aux).read_u32::<LittleEndian>().chain_err(|| {
            format!(
                "(Msg::header) Error at u32 parse for payloadchk for value {:?}",
                aux
            )
        })?;
        Ok(Header {
            network,
            cmd,
            payload_len,
            payloadchk,
        })
    }
}

impl std::fmt::Debug for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        let mut s = "Message Header:\n".to_string();
        s += &format!("├ Message Network Identification: {}\n", self.network);
        s += &format!(
            "├ Message Command OP_CODE: {:?}\n",
            self.cmd.clone().into_iter().collect::<Bytes>()
        );
        //str::from_utf8
        s += &format!("├ Payload Length: {}\n", self.payload_len);
        s += &format!("├ Payload Checksum: {}\n", self.payloadchk);

        write!(f, "{}", s)
    }
}

impl IntoBytes for Header {
    fn into_bytes(&self) -> Result<Vec<u8>> {
        let mut wtr = vec![];
        wtr.write_u32::<LittleEndian>(self.network)
            .chain_err(|| format!("Failure to convert network ({}) into byte vec", self.network))?;

        wtr.append(&mut self.cmd.to_vec());
            //.chain_err(|| format!("Failure to convert cmd ({}) into byte vec", self.cmd))?;

        wtr.write_i32::<LittleEndian>(self.payload_len)
            .chain_err(|| format!("Failure to convert payload_len ({}) into byte vec", self.payload_len))?;
        wtr.write_u32::<LittleEndian>(self.payloadchk)
            .chain_err(|| format!("Failure to convert payloadchk ({}) into byte vec", self.payloadchk))?;

        Ok(wtr)
    }
}