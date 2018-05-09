use arrayvec::ArrayVec;
use codec::msgs::msg::commons::bytes::Bytes;
use codec::msgs::msg::commons::new_from_hex::NewFromHex;
use codec::msgs::msg::commons::net_addr::NetAddr;
use std;
use std::fmt;
use std::io::Cursor;

use std::time::*;
use tokio_timer::*;
use chrono::Utc;

mod errors {
    error_chain!{}
}
use errors::*;
//use std::net::{IpAddr, Ipv4Addr,Ipv6Addr, SocketAddr};
use std::net::SocketAddr;

use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
use codec::msgs::msg::commons::into_bytes::IntoBytes;

// falta pub time: u32
// https://en.bitcoin.it/wiki/Protocol_documentation#Network_address

#[derive(Clone,Debug)]
pub struct NetAddrTime {
    /*A time in Unix epoch time format. 
    Nodes advertising their own IP address set this to the current time. 
    Nodes advertising IP addresses they’ve connected to set this to the last time they connected to that node. 
    Other nodes just relaying the IP address should not change the time. 
    Nodes can use the time field to avoid relaying old addr messages. 
    */
    pub time: u32,
    pub net_addr: NetAddr,
}

impl NewFromHex for NetAddrTime {
    fn new<'a, I>(it: I) -> Result<NetAddrTime>
    where
        I: IntoIterator<Item = &'a u8>,
    {
        let mut it = it.into_iter();
        let aux = it.by_ref().take(4).cloned().collect::<Vec<u8>>();
        let time = Cursor::new(&aux)
            .read_u32::<LittleEndian>()
            .chain_err(cf!("Error at u64 parse for service for value {:?}", aux))?;
        let net_addr = NetAddr::new(it.by_ref())?;
        
        Ok(NetAddrTime { time,net_addr})
    }
}

impl NetAddrTime {
    pub fn from_socket_addr(addr: &SocketAddr) -> NetAddrTime {

        let time = Utc::now().timestamp() as u32;
        let net_addr = NetAddr::from_socket_addr(addr);
        (NetAddrTime {time, net_addr})
    }
}

impl IntoBytes for NetAddrTime {
    fn into_bytes(&self) -> Result<Vec<u8>> {
        let mut wtr = vec![];
        wtr.write_u32::<LittleEndian>(self.time).chain_err(cf!(
            "Failure to convert time ({}) into byte vec",
            self.time
        ))?;

        wtr.append(&mut self.net_addr.into_bytes()?);

        Ok(wtr)
    }
}
