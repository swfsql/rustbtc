/*
use arrayvec::ArrayVec;
use codec::msgs::msg::commons::bytes::Bytes;
use codec::msgs::msg::commons::new_from_hex::NewFromHex;
use std;
use std::fmt;
use std::io::Cursor;

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

#[derive(Clone)]
pub struct Block {
    pub magic: u32,
    



    pub service: u64,
    pub ip: ArrayVec<[u8; 16]>,
    pub port: u16,
}

impl NewFromHex for Block {
    fn new<'a, I>(it: I) -> Result<NetAddr>
    where
        I: IntoIterator<Item = &'a u8>,
    {
        let mut it = it.into_iter();
        let aux = it.by_ref().take(8).cloned().collect::<Vec<u8>>();
        let service = Cursor::new(&aux)
            .read_u64::<LittleEndian>()
            .chain_err(cf!("Error at u64 parse for service for value {:?}", aux))?;
        let ip = it
            .by_ref()
            .take(16)
            //.map(|u| u.to_le())
            .cloned()
            .collect::<ArrayVec<[u8; 16]>>();
        let aux = it.by_ref().take(2).cloned().collect::<Vec<u8>>();
        let port = Cursor::new(&aux)
            .read_u16::<BigEndian>()
            .chain_err(cf!("Error at u16 parse for port for value {:?}", aux))?;
        Ok(NetAddr { service, ip, port })
    }
}

impl Block {
    pub fn from_socket_addr(addr: &SocketAddr) -> NetAddr {
        match *addr {
            SocketAddr::V4(socket_addr) => NetAddr {
                service: 0_u64,
                ip: ArrayVec::from(socket_addr.ip().to_ipv6_mapped().octets()),
                port: socket_addr.port(),
            },
            SocketAddr::V6(socket_addr) => NetAddr {
                service: 0_u64,
                ip: ArrayVec::from(socket_addr.ip().octets()),
                port: socket_addr.port(),
            },
        }
    }
}

impl std::fmt::Debug for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        let mut s = "Net Addr:\n".to_string();
        s += &format!("├ Service: {}\n", self.service);
        s += &format!(
            "├ IP: {:?}\n",
            self.ip.clone().into_iter().collect::<Bytes>()
        );
        s += &format!("├ Port: {}", self.port);
        write!(f, "{}", s)
    }
}

impl IntoBytes for Block {
    fn into_bytes(&self) -> Result<Vec<u8>> {
        let mut wtr = vec![];
        wtr.write_u64::<LittleEndian>(self.service).chain_err(cf!(
            "Failure to convert service ({}) into byte vec",
            self.service
        ))?;
        wtr.append(&mut self.ip.to_vec());
        wtr.write_u16::<BigEndian>(self.port)
            .chain_err(cf!("Failure to convert port ({}) into byte vec", self.port))?;
        Ok(wtr)
    }
}
*/