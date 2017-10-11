use std;
use std::iter::Iterator;
extern crate hex;

use hex::FromHex;
mod errors {
    error_chain!{}
}
use errors::*;

pub trait NewFromHex {
  fn new_from_hex(hex: &str) -> Result<Self>
  where Self: std::marker::Sized {
    let vec: Vec<u8> = Vec::from_hex(hex)
      .chain_err(|| "Error at from_hex for vec")?;
    let mut it = vec.into_iter();
    Self::new(it.by_ref())
  }
  fn new(it: &mut std::vec::IntoIter<u8>) -> Result<Self>
  where Self: std::marker::Sized;
}

