mod MsgHeader;
mod MsgPayload;

pub struct Msg {
  pub header: MsgHeader,
  pub payload: Option<MsgPayload>,
}

impl NewFromHex for Msg {
  fn new(it: &mut std::vec::IntoIter<u8>) -> Result<Msg, Box<Error>> {
    let header = MsgHeader::new(it).unwrap();
    let cmd_str = header.cmd.clone().into_iter()
      .map(|x| x as char).collect::<String>();

    let payload = match cmd_str.to_string().trim().as_ref() {

      "tx\0\0\0\0\0\0\0\0\0\0" => Some(MsgPayload::Tx(Tx::new(it).unwrap())),
      "ping\0\0\0\0\0\0\0\0" => Some(MsgPayload::Ping(Ping::new(it).unwrap())),
      "pong\0\0\0\0\0\0\0\0" => Some(MsgPayload::Pong(Pong::new(it).unwrap())),
      "version\0\0\0\0\0" => Some(MsgPayload::Version(Version::new(it).unwrap())),
      "verack\0\0\0\0\0\0" => Some(MsgPayload::Verack),
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
            &MsgPayload::Tx(ref tx) => format!("{:?}", tx),
            &MsgPayload::Ping(ref ping) => format!("{:?}", ping),
            &MsgPayload::Pong(ref pong) => format!("{:?}", pong),
            &MsgPayload::Version(ref version) => format!("{:?}", version),
            &MsgPayload::Verack => format!("Verack"),
          },
          None => "None".to_string(),
        }.lines().map(|x| "│ ".to_string() + x + "\n").collect::<String>();
      write!(f, "{}", s)
  }
}
