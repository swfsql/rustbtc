pub mod get_headers;
pub mod headers;
pub mod ping;
pub mod pong;
pub mod tx;
pub mod version;
//use codec::msgs::msg::commons::into_bytes::IntoBytes;
//use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

#[derive(Clone)]
pub enum Payload {
    Tx(tx::Tx),
    Ping(ping::Ping),
    Pong(pong::Pong),
    Version(version::Version),
    Verack,
    GetHeaders(get_headers::GetHeaders),
    Headers(headers::Headers)
}

/*
impl IntoBytes for Ping {
fn into_bytes(&self) -> Result<Vec<u8>> {
        let mut wtr = vec![];
        wtr.write_u64::<LittleEndian>(self.nonce)
            .chain_err(|| format!("Failure to convert nonce ({}) into byte vec", self.nonce))?;
        Ok(wtr)
    }
}
*/
