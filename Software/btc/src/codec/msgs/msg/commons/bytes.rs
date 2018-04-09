use std;
use std::fmt;
#[derive(Clone)]
pub struct Bytes(Vec<u8>);
// use std::ascii::AsciiExt;
mod errors {
    error_chain!{}
}
use errors::*;
use codec::msgs::msg::commons::into_bytes::IntoBytes;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

impl Bytes {
    pub fn new(vec: Vec<u8>) -> Bytes {
        Bytes(vec)
    }
}

impl std::iter::FromIterator<u8> for Bytes {
    fn from_iter<I: IntoIterator<Item = u8>>(iter: I) -> Self {
        let mut b = Bytes(Vec::new());
        for i in iter {
            b.0.push(i);
        }
        b
        // Bytes(iter.collect::<Vec<u8>>())
    }
}
impl std::fmt::Debug for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        let mut all_ascii = true;
        let as_char = self.0
            .iter()
            .map(|c| {
                if (*c as char).is_ascii() {
                    *c as char
                } else {
                    all_ascii = false;
                    '.'
                }
            })
            .collect::<String>();

        let o = self.0
            .iter()
            .enumerate()
            .map(|(i, s)| {
                if i % 4 == 0 {
                    if i % 8 == 0 {
                        if i % 32 == 0 {
                            if i == 0 {
                                format!("\n│ ├{:>4},", s)
                            } else {
                                format!("\n│ │\n│ ├{:>4},", s)
                            }
                        } else {
                            format!("\n│ ├{:>4},", s)
                        }
                    } else {
                        format!("{:>5},", s)
                    }
                } else {
                    format!("{:>4},", s)
                }
            })
            .collect::<String>() + "\n│ │";

        if all_ascii {
            write!(f, "<{}>{}", as_char, o)
        } else {
            write!(f, "{}", o)
        }
    }
}


impl IntoBytes for Bytes {
    fn into_bytes(&self) -> Result<Vec<u8>> {
        let mut wtr = self.0.clone();
        Ok(wtr)
    }
}
