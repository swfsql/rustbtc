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

// uses Ping::new()
#[test]
fn ping_payload() {
    let ping_pl_hex = "\
      0094102111e2af4d\
    ";
    let expected = "\
      Ping:\n\
      ├ Nonce: 5597941425041871872\n\
    ";

    let payload_vec: Result<Vec<u8>> =
        Vec::from_hex(ping_pl_hex.trim()).chain_err(|| "Fail in hex -> Vec<u8>");
    // let payload_vec: Vec<u8> = payload_vec.clone().expect(&payload_vec.unwrap_err().display_chain().to_string());
    let payload_vec: Vec<u8> = unwrap_or_display(payload_vec);

    let payload = btc::msg::payload::ping::Ping::new(payload_vec.into_iter().by_ref())
        .chain_err(|| "Fail in hex -> Msg when testing Ping Payload");
    // let payload = payload.clone().expect(&payload.unwrap_err().display_chain().to_string());
    let payload = unwrap_or_display(payload);
    let res = format!("{:?}", payload);

    println!("{}", res);
    assert_eq!(expected.trim(), res.trim());
}

// uses Msg::new_from_hex()
// tries to extract some (transaction) payload information from a different kind (ping) of message
#[test]
fn ping_msg() {
    let ping_msg_hex = "\
      F9BEB4D9\
      70696E670000000000000000\
      0800000088EA8176\
      0094102111e2af4d\
    ";
    let expected = "\
      Message:\n\
      ├ Message Header: Message Header:\n\
      ├ Message Network Identification: 3652501241\n\
      ├ Message Command OP_CODE: <ping\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}>\n\
      │ ├ 112, 105, 110, 103,    0,   0,   0,   0,\n\
      │ ├   0,   0,   0,   0,\n\
      │ │\n\
      ├ Payload Length: 8\n\
      ├ Payload Checksum: 1988225672\n\
      ├ Message Payload: \n\
      │ Ping:\n\
      │ ├ Nonce: 5597941425041871872\n\
    ";

    let msg_ping = btc::msg::Msg::new_from_hex(&ping_msg_hex)
        .chain_err(|| "Fail in hex -> Msg");
    let msg_ping = unwrap_or_display(msg_ping);
    let res = format!("{:?}", &msg_ping);

    println!("{}", res);
    assert_eq!(expected.trim(), res.trim());

    // tries to extract a transaction from the payload, of a ping message (should give None)
    let expected = "\
      None\n\
    ";

    let tx = if let Some(btc::msg::payload::Payload::Tx(tx)) = msg_ping.payload {
        Some(tx)
    } else {
        None
    };
    let res = format!("{:?}", &tx);

    println!("{}", res);
    assert_eq!(expected.trim(), res.trim());
}

// uses Msg::new_from_hex()
#[test]
fn pong_msg() {
    let pong_msg_hex = "\
       F9BEB4D9\
       706F6E670000000000000000\
       08000000EAF3B51D\
       0194102111e2af4d\
    ";
    let expected = "\
      Message:\n\
      ├ Message Header: Message Header:\n\
      ├ Message Network Identification: 3652501241\n\
      ├ Message Command OP_CODE: <pong\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}>\n\
      │ ├ 112, 111, 110, 103,    0,   0,   0,   0,\n\
      │ ├   0,   0,   0,   0,\n\
      │ │\n\
      ├ Payload Length: 8\n\
      ├ Payload Checksum: 498463722\n\
      ├ Message Payload: \n\
      │ Pong:\n\
      │ ├ Nounce: 5597941425041871873\n\
    ";

    let msg_pong = btc::msg::Msg::new_from_hex(&pong_msg_hex)
        .chain_err(|| "Fail in hex -> Msg");
    let msg_pong = unwrap_or_display(msg_pong);
    let res = format!("{:?}", &msg_pong);

    println!("{}", res);
    assert_eq!(expected.trim(), res.trim());
}


// uses Msg::new_from_hex()
#[test]
fn version_msg() {
    let version_msg_hex = "\
    F9BEB4D976657273696F6E0000000000640000003B648D5A\
    62EA000001000000000000001\
    1B2D050000000000100000000\
    0000000000000000000000000\
    0FFFF00000000000001000000\
    0000000000000000000000000\
    000FFFF0000000000003B2EB3\
    5D8CE617650F2F5361746F736\
    8693A302E372E322FC03E0300\
    ";
    let expected = "\
      Message:\n\
      ├ Message Header: Message Header:\n\
      ├ Message Network Identification: 3652501241\n\
      ├ Message Command OP_CODE: <version\u{0}\u{0}\u{0}\u{0}\u{0}>\n\
      │ ├ 118, 101, 114, 115,  105, 111, 110,   0,\n\
      │ ├   0,   0,   0,   0,\n\
      │ │\n\
      ├ Payload Length: 100\n\
      ├ Payload Checksum: 1519215675\n\
      ├ Message Payload: \n\
      │ Version:\n\
      │ ├ Version: 60002\n\
      │ ├ Services: 1\n\
      │ ├ Timestamp: 1355854353\n\
      │ ├ Addr Receiver: Net Addr:\n\
      │ ├ Service: 1\n\
      │ ├ IP: \n\
      │ │ ├   0,   0,   0,   0,    0,   0,   0,   0,\n\
      │ │ ├   0,   0, 255, 255,    0,   0,   0,   0,\n\
      │ │ │\n\
      │ ├ Port: 0\n\
      │ ├ Addr Transfer: Net Addr:\n\
      │ ├ Service: 1\n\
      │ ├ IP: \n\
      │ │ ├   0,   0,   0,   0,    0,   0,   0,   0,\n\
      │ │ ├   0,   0, 255, 255,    0,   0,   0,   0,\n\
      │ │ │\n\
      │ ├ Port: 0\n\
      │ ├ Nonce: 7284544412836900411\n\
      │ ├ User Agent: Version:\n\
      │ ├ Length: U8(15)\n\
      │ ├ String: </Satoshi:0.7.2/>\n\
      │ │ ├  47,  83,  97, 116,  111, 115, 104, 105,\n\
      │ │ ├  58,  48,  46,  55,   46,  50,  47,\n\
      │ │ │\n\
      │ ├ Start Height: 212672\n\
      │ ├ Relay: None\n\
    ";

    let msg_version = btc::msg::Msg::new_from_hex(&version_msg_hex)
        .chain_err(|| "Fail in hex -> Msg");
    let msg_version = unwrap_or_display(msg_version);
    let res = format!("{:?}", &msg_version);

    println!("{}", res);
    assert_eq!(expected.trim(), res.trim());
}

// uses Msg::new_from_hex()
#[test]
fn tx_msg() {
    let tx_msg_hex = "\
      F9BEB4D9\
      747800000000000000000000\
      02010000E293CDBE\
      01000000016DBDDB085B1D8A\
      F75184F0BC01FAD58D1266E9\
      B63B50881990E4B40D6AEE36\
      29000000008B483045022100\
      F3581E1972AE8AC7C7367A7A\
      253BC1135223ADB9A468BB3A\
      59233F45BC578380022059AF\
      01CA17D00E41837A1D58E97A\
      A31BAE584EDEC28D35BD9692\
      3690913BAE9A0141049C02BF\
      C97EF236CE6D8FE5D94013C7\
      21E915982ACD2B12B65D9B7D\
      59E20A842005F8FC4E02532E\
      873D37B96F09D6D4511ADA8F\
      14042F46614A4C70C0F14BEF\
      F5FFFFFFFF02404B4C000000\
      00001976A9141AA0CD1CBEA6\
      E7458A7ABAD512A9D9EA1AFB\
      225E88AC80FAE9C700000000\
      1976A9140EAB5BEA436A0484\
      CFAB12485EFDA0B78B4ECC52\
      88AC00000000";
    let expected = "\
      Message:\n\
      ├ Message Header: Message Header:\n\
      ├ Message Network Identification: 3652501241\n\
      ├ Message Command OP_CODE: <tx\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}>\n\
      │ ├ 116, 120,   0,   0,    0,   0,   0,   0,\n\
      │ ├   0,   0,   0,   0,\n\
      │ │\n\
      ├ Payload Length: 258\n\
      ├ Payload Checksum: 3201143778\n\
      ├ Message Payload: \n\
      │ Tx:\n\
      │ ├ Version: 1\n\
      │ ├ Inputs Length: 1\n\
      │ ├ Inputs:\n\
      │ │ ├ Input0:\n\
      │ │ │ ├ Previous Tx: \n\
      │ │ │ │ ├ 109, 189, 219,   8,   91,  29, 138, 247,\n\
      │ │ │ │ ├  81, 132, 240, 188,    1, 250, 213, 141,\n\
      │ │ │ │ ├  18, 102, 233, 182,   59,  80, 136,  25,\n\
      │ │ │ │ ├ 144, 228, 180,  13,  106, 238,  54,  41,\n\
      │ │ │ │ │\n\
      │ │ │ ├ Previous Tx Out Index: 0\n\
      │ │ │ ├ Script Length: 139\n\
      │ │ │ ├ Script Signature: \n\
      │ │ │ │ ├  72,  48,  69,   2,   33,   0, 243,  88,\n\
      │ │ │ │ ├  30,  25, 114, 174,  138, 199, 199,  54,\n\
      │ │ │ │ ├ 122, 122,  37,  59,  193,  19,  82,  35,\n\
      │ │ │ │ ├ 173, 185, 164, 104,  187,  58,  89,  35,\n\
      │ │ │ │ │\n\
      │ │ │ │ ├  63,  69, 188,  87,  131, 128,   2,  32,\n\
      │ │ │ │ ├  89, 175,   1, 202,   23, 208,  14,  65,\n\
      │ │ │ │ ├ 131, 122,  29,  88,  233, 122, 163,  27,\n\
      │ │ │ │ ├ 174,  88,  78, 222,  194, 141,  53, 189,\n\
      │ │ │ │ │\n\
      │ │ │ │ ├ 150, 146,  54, 144,  145,  59, 174, 154,\n\
      │ │ │ │ ├   1,  65,   4, 156,    2, 191, 201, 126,\n\
      │ │ │ │ ├ 242,  54, 206, 109,  143, 229, 217,  64,\n\
      │ │ │ │ ├  19, 199,  33, 233,   21, 152,  42, 205,\n\
      │ │ │ │ │\n\
      │ │ │ │ ├  43,  18, 182,  93,  155, 125,  89, 226,\n\
      │ │ │ │ ├  10, 132,  32,   5,  248, 252,  78,   2,\n\
      │ │ │ │ ├  83,  46, 135,  61,   55, 185, 111,   9,\n\
      │ │ │ │ ├ 214, 212,  81,  26,  218, 143,  20,   4,\n\
      │ │ │ │ │\n\
      │ │ │ │ ├  47,  70,  97,  74,   76, 112, 192, 241,\n\
      │ │ │ │ ├  75, 239, 245,\n\
      │ │ │ │ │\n\
      │ │ │ ├ Sequence: 4294967295\n\
      │ ├ Outputs Length: 2\n\
      │ ├ Outputs:\n\
      │ │ ├ Output0:\n\
      │ │ │ ├ Value: 5000000\n\
      │ │ │ ├ PubKey Script Length: 25\n\
      │ │ │ ├ PubKey Script: \n\
      │ │ │ │ ├ 118, 169,  20,  26,  160, 205,  28, 190,\n\
      │ │ │ │ ├ 166, 231,  69, 138,  122, 186, 213,  18,\n\
      │ │ │ │ ├ 169, 217, 234,  26,  251,  34,  94, 136,\n\
      │ │ │ │ ├ 172,\n\
      │ │ │ │ │\n\
      │ │ ├ Output1:\n\
      │ │ │ ├ Value: 3354000000\n\
      │ │ │ ├ PubKey Script Length: 25\n\
      │ │ │ ├ PubKey Script: \n\
      │ │ │ │ ├ 118, 169,  20,  14,  171,  91, 234,  67,\n\
      │ │ │ │ ├ 106,   4, 132, 207,  171,  18,  72,  94,\n\
      │ │ │ │ ├ 253, 160, 183, 139,   78, 204,  82, 136,\n\
      │ │ │ │ ├ 172,\n\
      │ │ │ │ │\n\
      │ ├ Locktime: 0\n\
      ";


    let msg_tx = btc::msg::Msg::new_from_hex(&tx_msg_hex)
        .chain_err(|| "Fail in hex -> Msg");
    let msg_tx = unwrap_or_display(msg_tx);
    assert_eq!(expected.trim(), format!("{:?}", &msg_tx).trim());

    // this is how to access only the tx (from the payload)
    // let tx = if let Some(btc::msg::payload::Payload::Tx(tx)) = msg_tx.payload {
    //     Some(tx)
    // } else {
    //     None
    // };

}

