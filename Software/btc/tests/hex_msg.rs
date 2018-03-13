#![recursion_limit = "1024"]
#[macro_use]
extern crate error_chain;
mod errors {
    error_chain!{}
}
use errors::*;
use error_chain::ChainedError;

extern crate hex;

extern crate btc;
use btc::commons::new_from_hex::NewFromHex;
// use btc::commons::into_bytes::IntoBytes;
use hex::FromHex;

fn unwrap_or_display<T>(res: Result<T>) -> T {
    match res {
        Err(e) => panic!("\n{}", e.display_chain().to_string()),
        Ok(o) => o,
    }
}

#[test]
fn ping_payload() {
    let ping_pl_hex = "
0094102111e2af4d
    ";
    let expected = "
Ping:
├ Nonce: 5597941425041871872
    ";

    let payload_vec: Result<Vec<u8>> =
        Vec::from_hex(ping_pl_hex.trim()).chain_err(|| "Fail in hex -> Vec<u8>");
    // let payload_vec: Vec<u8> = payload_vec.clone().expect(&payload_vec.unwrap_err().display_chain().to_string());
    let payload_vec: Vec<u8> = unwrap_or_display(payload_vec);

    let payload = btc::msg::payload::ping::Ping::new(payload_vec.into_iter().by_ref())
        .chain_err(|| "Fail in hex -> Msg when testing Ping Payload");
    // let payload = payload.clone().expect(&payload.unwrap_err().display_chain().to_string());
    let payload = unwrap_or_display(payload);

    assert_eq!(expected.trim(), format!("{:?}", payload).trim());
}
