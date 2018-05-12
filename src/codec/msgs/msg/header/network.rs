use std::fmt;
use std;
use byteorder::{LittleEndian, WriteBytesExt};
use codec::msgs::msg::commons::into_bytes::IntoBytes;

mod errors {
    error_chain!{}
}
use errors::*;

mod network_value {
    pub const MAIN: u32 = 0xD9B4BEF9;
    pub const TESTNET: u32 = 0xDAB5BFFA;
    pub const TESTNET3: u32 = 0x0709110B;
    pub const NAMECOIN: u32 = 0xFEB4BEF9;
}

#[derive(Clone)]
pub enum Network {
    Main,
    Testnet,
    Testnet3,
    Namecoin,
}

impl Network {
    pub fn new(magic: u32) -> Option<Network> {
        match magic {
            network_value::MAIN => Some(Network::Main),
            network_value::TESTNET => Some(Network::Testnet),
            network_value::TESTNET3 => Some(Network::Testnet3),
            network_value::NAMECOIN => Some(Network::Namecoin),
            _ => None,
        }
    }

    pub fn value(&self) -> u32 {
        match *self {
            Network::Main => network_value::MAIN,
            Network::Testnet => network_value::TESTNET,
            Network::Testnet3 => network_value::TESTNET3,
            Network::Namecoin => network_value::NAMECOIN,
        }
    }
}

impl std::fmt::Debug for Network {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        let s = match *self {
            Network::Main => format!("Main <{:X}>\n", self.value()),
            Network::Testnet => format!("Testnet <{:X}>\n", self.value()),
            Network::Testnet3 => format!("Testnet3 <{:X}>\n", self.value()),
            Network::Namecoin => format!("Namecoin <{:X}>\n", self.value()),
        };
        write!(f, "{}", s)
    }
}

impl IntoBytes for Network {
    fn into_bytes(&self) -> Result<Vec<u8>> {
        let mut wtr = vec![];
        wtr.write_u32::<LittleEndian>(self.value()).chain_err(cf!(
            "Failure to convert network ({}) into byte vec",
            self.value()
        ))?;
        Ok(wtr)
    }
}