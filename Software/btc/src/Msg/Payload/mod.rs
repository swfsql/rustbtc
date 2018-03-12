
pub mod Tx;
pub mod Ping;
pub mod Pong;
pub mod Version;

pub enum Payload {
  Tx(Tx::Tx),
  Ping(Ping::Ping),
  Pong(Pong::Pong),
  Version(Version::Version),
  Verack,
}

