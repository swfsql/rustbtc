mod Tx;
mod Ping;
mod Pong;
mod Version;

pub enum MsgPayload {
  Tx(Tx),
  Ping(Ping),
  Pong(Pong),
  Version(Version),
  Verack,
}

