use std;
use std::fmt;
use codec::msgs::msg::commons::bytes::Bytes;
use arrayvec::ArrayVec;
use codec::msgs::msg::commons::new_from_hex::NewFromHex;
use std::io::Cursor;

mod errors {
    error_chain!{}
}
use errors::*;
use std::net::{IpAddr, Ipv4Addr,Ipv6Addr, SocketAddr};

use codec::msgs::msg::commons::into_bytes::IntoBytes;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

// falta pub time: u32
// https://en.bitcoin.it/wiki/Protocol_documentation#Network_address

#[derive(Clone)]
pub struct NetAddr {
    pub service: u64,
    pub ip: ArrayVec<[u8; 16]>,
    pub port: u16,
}

impl NewFromHex for NetAddr {
    fn new(it: &mut std::vec::IntoIter<u8>) -> Result<NetAddr> {
        let aux = it.by_ref().take(8).collect::<Vec<u8>>();
        let service = Cursor::new(&aux).read_u64::<LittleEndian>().chain_err(|| {
            format!(
                "(Commons::net_addr) Error at u64 parse for service for value {:?}",
                aux
            )
        })?;
        let ip = it.by_ref()
            .take(16)
            .map(|u| u.to_le())
            .collect::<ArrayVec<[u8; 16]>>();
        let aux = it.by_ref().take(2).collect::<Vec<u8>>();
        let port = Cursor::new(&aux).read_u16::<LittleEndian>().chain_err(|| {
            format!(
                "(Commons::net_addr) Error at u16 parse for port for value {:?}",
                aux
            )
        })?;
        Ok(NetAddr { service, ip, port })
    }
}

impl NetAddr {
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

impl std::fmt::Debug for NetAddr {
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


impl IntoBytes for NetAddr {
    fn into_bytes(&self) -> Result<Vec<u8>> {
        let mut wtr = vec![];
        wtr.write_u64::<LittleEndian>(self.service)
            .chain_err(|| format!("Failure to convert service ({}) into byte vec", self.service))?;
        wtr.append(&mut self.ip.to_vec());
        wtr.write_u16::<LittleEndian>(self.port)
            .chain_err(|| format!("Failure to convert port ({}) into byte vec", self.port))?;
        Ok(wtr)
    }
}
