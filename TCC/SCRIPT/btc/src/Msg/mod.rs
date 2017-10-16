use std;
use std::fmt;
use Commons::NewFromHex::NewFromHex;
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
