use std;
use std::fmt;
use Commons::NewFromHex::NewFromHex;
use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};
extern crate crypto;

use self::crypto::digest::Digest;
use self::crypto::sha2::Sha256;

mod errors {
    error_chain!{}
}
use errors::*;

//use ::Payload::Payload::Verack;
//use Msg::Payload::Payload::Verack;

pub mod Header;
pub mod Payload;

pub struct Msg {
  pub header: Header::Header,
  pub payload: Option<Payload::Payload>,
}

impl NewFromHex for Msg {
  fn new(it: &mut std::vec::IntoIter<u8>) -> Result<Msg> {

    let header = Header::Header::new(it)
      .chain_err(|| "(Msg) Error at creating Header")?;
    let cmd_str = header.cmd.clone().into_iter()
      .map(|x| x as char).collect::<String>();

    let (last_index,payload_arrvec): (i32, Vec<u8>) = it.clone()
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
    &chk.result(&mut sha);
    chk.reset();
    chk.input(&sha);
    chk.result(&mut sha);

    let chk = Cursor::new(&sha)
        .read_u32::<LittleEndian>()
        .chain_err(|| format!("(Msg::mod) Error at u32 parse for payloadchk for value {:?}", &sha))?;

    if last_index + 1 != header.payload_len {
      bail!("(Msg::mod) Error at payload length (expected: {}, found: {:?})", header.payload_len, last_index + 1);
    } else if chk != header.payloadchk {
      bail!("(Msg::mod) Error at payload checksum (expected: {}, found: {:?})", header.payloadchk, &chk);
    }

     //println!("Só ALEDGRIA4444");

    let payload = match cmd_str.to_string().trim().as_ref() {

      "tx\0\0\0\0\0\0\0\0\0\0" => {
        let tx = Payload::Tx::Tx::new(it)
          .chain_err(|| "(Msg) Error at creating Payload")?;
        Some(Payload::Payload::Tx(tx))
      },
      "ping\0\0\0\0\0\0\0\0" => {
        let ping = Payload::Ping::Ping::new(it)
          .chain_err(|| "(Msg) Error at creating ping")?;
        Some(Payload::Payload::Ping(ping))
      },
      "pong\0\0\0\0\0\0\0\0" => {
        let pong = Payload::Pong::Pong::new(it)
          .chain_err(|| "(Msg) Error at creating pong")?;
        Some(Payload::Payload::Pong(pong))
      },
      "version\0\0\0\0\0" => {
        let version = Payload::Version::Version::new(it)
          .chain_err(|| "(Msg) Error at creating version")?;
        Some(Payload::Payload::Version(version))
      },
      "verack\0\0\0\0\0\0" => {
        Some(Payload::Payload::Verack)
      },
      x => {
        println!("payload code didnt match: {:?}", x);
        None
      },
    };

    // header.payload_len // TODO


    Ok(Msg {
      header: header,
      payload: payload,
    })
  }
}

impl std::fmt::Debug for Msg {
  fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
      let mut s = "Message:\n".to_string();
      s += &format!("├ Message Header: {:?}", self.header);
      s += &"├ Message Payload: \n".to_string();
      s += &match self.clone().payload {
          Some(ref p) => match p {
            &Payload::Payload::Tx(ref tx) => format!("{:?}", tx),
            &Payload::Payload::Ping(ref ping) => format!("{:?}", ping),
            &Payload::Payload::Pong(ref pong) => format!("{:?}", pong),
            &Payload::Payload::Version(ref version) => format!("{:?}", version),
            &Payload::Payload::Verack => format!("Verack"),
          },
          None => "None".to_string(),
        }.lines().map(|x| "│ ".to_string() + x + "\n").collect::<String>();
      write!(f, "{}", s)
  }
}
