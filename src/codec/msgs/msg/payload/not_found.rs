//use arrayvec::ArrayVec;
//use std;
//use std::fmt;
//use std::io::Cursor;

//use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
//use codec::msgs::msg::commons::{net_addr, new_from_hex, var_str};

//use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use codec::msgs::msg::commons::into_bytes::IntoBytes;
use codec::msgs::msg::commons::new_from_hex::NewFromHex;
use codec::msgs::msg::commons::var_uint::VarUint;
use codec::msgs::msg::commons::inventory::Inventory;
//use std::io::Cursor;
mod errors {
    error_chain!{}
}
use errors::*;

// https://bitcoin.org/en/developer-reference#ping

#[derive(Clone,Debug)]
pub struct NotFound {
    pub count: VarUint,
    pub inventories: Vec<Inventory>,
}

impl NewFromHex for NotFound {
    fn new<'a, I>(it: I) -> Result<NotFound>
    where
        I: IntoIterator<Item = &'a u8>,
    {
        let mut it = it.into_iter();

        let count = VarUint::new(it.by_ref())
            .chain_err(cf!("Error at new VarUint for length"))?;

        let count_usize = count
            .as_usize()
            .ok_or(ff!("Error at creating count length: too big"))?;

        let mut inventories: Vec<Inventory> = vec![];
        for i in 0..count_usize {
            let aux = Inventory::new(&mut it)
                .chain_err(cf!("Error at creating a new Inventory, at NotFoundentories {:?}", i))?;
            inventories.push(aux);
        }

        Ok(NotFound {
            count,
            inventories,
        })
    }
}

impl IntoBytes for NotFound {
    fn into_bytes(&self) -> Result<Vec<u8>> {
        let mut wtr = vec![];
        
        let mut count_vec = self.count.into_bytes()
            .chain_err(cf!(
                "Failure to convert count ({:?}) into byte vec",
                self.count))?;
        wtr.append(&mut count_vec);
        //.chain_err(cf!("Failure to convert cmd ({}) into byte vec", self.cmd))?;

        let inventories = self.inventories
            .iter()
            .map(|inventory| inventory.into_bytes())
            .collect::<Result<Vec<_>>>()?;

        for mut inventory in inventories {
            wtr.append(&mut inventory);
        }

        Ok(wtr)
    }
}
