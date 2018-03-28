// use std;

mod errors {
    error_chain!{}
}
use errors::*;

pub trait IntoBytes {
    fn into_bytes(&self) -> Result<Vec<u8>>;
}
