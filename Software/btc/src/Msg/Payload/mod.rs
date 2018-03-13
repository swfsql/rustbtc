
pub mod tx;
pub mod ping;
pub mod pong;
pub mod version;

pub enum Payload {
  Tx(tx::Tx),
  Ping(ping::Ping),
  Pong(pong::Pong),
  Version(version::Version),
  Verack,
}

