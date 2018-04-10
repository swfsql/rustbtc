use std;
use std::fmt;
use codec::msgs::msg::commons::new_from_hex::NewFromHex;
use codec::msgs::msg::commons::into_bytes::IntoBytes;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
// use codec::msgs::msg::commons::into_bytes::into_bytes;
use std::io::Cursor;

extern crate crypto;

use self::crypto::digest::Digest;
use self::crypto::sha2::Sha256;

mod errors {
    error_chain!{}
}
use errors::*;
//use ::payload::payload::Verack;
//use codec::msgs::msg::payload::payload::Verack;

pub mod header;
pub mod payload;
pub mod commons;

#[derive(Clone)]
pub struct Msg {
    pub header: header::Header,
    pub payload: Option<payload::Payload>,
}

impl NewFromHex for Msg {
    fn new(it: &mut std::vec::IntoIter<u8>) -> Result<Msg> {
        let header = header::Header::new(it).chain_err(|| "(Msg) Error at creating Header")?;
        let cmd_str = header
            .cmd
            .clone()
            .into_iter()
            .map(|x| x as char)
            .collect::<String>();

        let (last_index, payload_arrvec): (i32, Vec<u8>) = it.clone()
      .collect::<Vec<u8>>().iter()
      .enumerate()
      .take(header.payload_len as usize)
      // initiate at -1i32 exclusively for the empty payload case
      .fold((-1i32, Vec::new()), |(_, mut acc), (i, hex)| {
        acc.push(*hex);
        (i as i32, acc)
      });

        let mut sha = [0; 32];
        let mut chk = Sha256::new();
        chk.input(payload_arrvec.as_slice());
        chk.result(&mut sha);
        chk.reset();
        chk.input(&sha);
        chk.result(&mut sha);

        let chk = Cursor::new(&sha).read_u32::<LittleEndian>().chain_err(|| {
            format!(
                "(Msg::mod) Error at u32 parse for payloadchk for value {:?}",
                &sha
            )
        })?;

        if last_index + 1 != header.payload_len {
            bail!(
                "(Msg::mod) Error at payload length (expected: {}, found: {:?})",
                header.payload_len,
                last_index + 1
            );
        } else if chk != header.payloadchk {
            bail!(
                "(Msg::mod) Error at payload checksum (expected: {}, found: {:?})",
                header.payloadchk,
                &chk
            );
        }

        //i!("Só ALEDGRIA4444");

        let payload = match cmd_str.to_string().trim() {
            "tx\0\0\0\0\0\0\0\0\0\0" => {
                let tx = payload::tx::Tx::new(it).chain_err(|| "(Msg) Error at creating Payload")?;
                Some(payload::Payload::Tx(tx))
            }
            "ping\0\0\0\0\0\0\0\0" => {
                let ping =
                    payload::ping::Ping::new(it).chain_err(|| "(Msg) Error at creating ping")?;
                Some(payload::Payload::Ping(ping))
            }
            "pong\0\0\0\0\0\0\0\0" => {
                let pong =
                    payload::pong::Pong::new(it).chain_err(|| "(Msg) Error at creating pong")?;
                Some(payload::Payload::Pong(pong))
            }
            "version\0\0\0\0\0" => {
                let version = payload::version::Version::new(it)
                    .chain_err(|| "(Msg) Error at creating version")?;
                Some(payload::Payload::Version(version))
            }
            "verack\0\0\0\0\0\0" => Some(payload::Payload::Verack),
            x => {
                i!("payload code didnt match: {:?}", x);
                None
            }
        };

        // header.payload_len // TODO

        Ok(Msg { header, payload })
    }
}

impl std::fmt::Debug for Msg {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        let mut s = "Message:\n".to_string();
        s += &format!("├ Message Header: {:?}", self.header);
        s += &"├ Message Payload: \n".to_string();
        s += &match self.payload {
            Some(ref p) => match *p {
                payload::Payload::Tx(ref tx) => format!("{:?}", tx),
                payload::Payload::Ping(ref ping) => format!("{:?}", ping),
                payload::Payload::Pong(ref pong) => format!("{:?}", pong),
                payload::Payload::Version(ref version) => format!("{:?}", version),
                payload::Payload::Verack => "Verack".into(),
            },
            None => "None".to_string(),
        }.lines()
            .map(|x| "│ ".to_string() + x + "\n")
            .collect::<String>();
        write!(f, "{}", s)
    }
}
impl IntoBytes for Msg {
  fn into_bytes(&self) -> Result<Vec<u8>> {
      let mut wrt = vec![];
      wrt.append(&mut self.header.into_bytes()?);
      let mut wrt_payload = match self.clone().payload {
          Some(ref p) => match p {
            &payload::Payload::Tx(ref tx) => tx.into_bytes()?,
            &payload::Payload::Ping(ref ping) => ping.into_bytes()?,
            &payload::Payload::Pong(ref pong) => pong.into_bytes()?,
            &payload::Payload::Version(ref version) => version.into_bytes()?,
            &payload::Payload::Verack => vec![],
          },
          None => vec![],
      };
      wrt.append(&mut wrt_payload);
      Ok(wrt)
  }
}
