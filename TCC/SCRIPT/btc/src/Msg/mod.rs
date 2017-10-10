use std;
use std::fmt;
use std::error::Error;
use Commons::NewFromHex::NewFromHex;
use byteorder::{LittleEndian, ReadBytesExt};

//use ::Payload::Payload::Verack;
//use Msg::Payload::Payload::Verack;

pub mod Header;
pub mod Payload;

pub struct Msg {
  pub header: Header::Header,
  pub payload: Option<Payload::Payload>,
}

impl NewFromHex for Msg {
  fn new(it: &mut std::vec::IntoIter<u8>) -> Result<Msg, Box<Error>> {
    let header = Header::Header::new(it).unwrap();
    let cmd_str = header.cmd.clone().into_iter()
      .map(|x| x as char).collect::<String>();

    let payload = match cmd_str.to_string().trim().as_ref() {

      "tx\0\0\0\0\0\0\0\0\0\0" => Some(Payload::Payload::Tx(Payload::Tx::Tx::new(it).unwrap())),
      "ping\0\0\0\0\0\0\0\0" => Some(Payload::Payload::Ping(Payload::Ping::Ping::new(it).unwrap())),
      "pong\0\0\0\0\0\0\0\0" => Some(Payload::Payload::Pong(Payload::Pong::Pong::new(it).unwrap())),
      "version\0\0\0\0\0" => Some(Payload::Payload::Version(Payload::Version::Version::new(it).unwrap())),
      "verack\0\0\0\0\0\0" => Some(Payload::Payload::Verack),
      _ => None,
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
