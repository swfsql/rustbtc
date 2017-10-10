// TODO compilar a caralha


//extern crate hex;
extern crate byteorder;
extern crate arrayvec;
use std::io::Cursor;
use std::fmt;
//use std::io::{Error, ErrorKind};
use std::error::Error;

use std::ascii::AsciiExt;

//use hex::FromHex;
use byteorder::{LittleEndian, ReadBytesExt};
use arrayvec::ArrayVec;

extern crate hex;

use hex::FromHex;
use std::iter::Iterator;

pub mod Msg;
pub mod Commons;



