//use std;
//use std::fmt;
//use std::io::Cursor;


//use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use codec::msgs::msg::commons::into_bytes::IntoBytes;
use codec::msgs::msg::commons::new_from_hex::NewFromHex;
use codec::msgs::msg::commons::{net_addr_time,  var_uint};
mod errors {
    error_chain!{}
}
use errors::*;

// https://en.bitcoin.it/wiki/Protocol_documentation#version
// https://bitcoin.org/en/developer-reference#version

#[derive(Clone,Debug)]
pub struct Addr {
    pub addr_count: var_uint::VarUint,
    pub addrs: Vec<net_addr_time::NetAddrTime>,
}

// https://bitcoin.org/en/developer-reference#protocol-versions
impl NewFromHex for Addr {
    fn new<'a, I>(it: I) -> Result<Addr>
    where
        I: IntoIterator<Item = &'a u8>,
    {
        let mut it = it.into_iter();

        let addr_count = var_uint::VarUint::new(it.by_ref())
            .chain_err(cf!("Error at new VarUint for length"))?;            

        let addr_count_usize = addr_count
            .as_usize()
            .ok_or(ff!("Error at creating HashCount length: too big"))?;

        let mut addrs: Vec<net_addr_time::NetAddrTime> = vec![];
        for i in 0..addr_count_usize {
            let aux =  net_addr_time::NetAddrTime::new(it.by_ref())
                .chain_err(cf!("Error at creating a new addr, at addrs {}", i))?;
            addrs.push(aux);
        }

        Ok(Addr {
            addr_count,
            addrs,
        })
    }
}

impl IntoBytes for Addr {
    fn into_bytes(&self) -> Result<Vec<u8>> {
        let mut wtr = vec![];
        
        let mut addr_count= self.addr_count.into_bytes()
            .chain_err(cf!(
                "Failure to convert addr_count ({:?}) into byte vec",
                self.addr_count))?;
        wtr.append(&mut addr_count);

        let addrs = self.addrs
            .iter()
            .map(|addrs| addrs.into_bytes())
            .collect::<Result<Vec<_>>>()?;

        for mut addrs in addrs {
            wtr.append(&mut addrs);
        }

        Ok(wtr)
    }
}
