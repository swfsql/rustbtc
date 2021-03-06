pub mod addr;
pub mod get_blocks;
pub mod block;
pub mod get_data;
pub mod get_headers;
pub mod headers;
pub mod inv;
pub mod not_found;
pub mod ping;
pub mod pong;
pub mod tx;
pub mod version;
pub mod fee_filter;
pub mod reject;
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
    Headers(headers::Headers),
    SendHeaders,
    GetAddr,
    Addr(addr::Addr),
    GetData(get_data::GetData),
    GetBlocks(get_blocks::GetBlocks),
    Block(block::Block),
    Inv(inv::Inv),
    NotFound(not_found::NotFound),
    FeeFilter(fee_filter::FeeFilter),
    MemPool,
    Reject(reject::Reject)
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
