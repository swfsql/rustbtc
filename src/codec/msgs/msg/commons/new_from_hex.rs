use std;
extern crate hex;

use hex::FromHex;
mod errors {
    error_chain!{}
}
use errors::*;

pub trait NewFromHex {
    fn new_from_hex(hex: &str) -> Result<Self>
    where
        Self: std::marker::Sized,
    {
        let vec: Vec<u8> = Vec::from_hex(hex).chain_err(cf!("Error at from_hex for vec"))?;
        let mut it = vec.iter();
        Self::new(&mut it)
    }
    fn new<'a, I>(it: I) -> Result<Self>
    where
        Self: std::marker::Sized,
        I: IntoIterator<Item = &'a u8>;
}
