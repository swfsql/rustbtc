
/*

TODO: fazer o sha256 funcionar

TODO: validar Msg de acordo com payload_len
TALVEZ: Validar de acordo com o checksum
*/

#![recursion_limit = "1024"]
#[macro_use] extern crate error_chain;
mod errors {
    error_chain!{}
}
use errors::*;


extern crate hex;

extern crate btc;
use btc::Commons::NewFromHex::NewFromHex;
use hex::FromHex;


fn run() -> Result<()> {
  let msg_tx_hex = "".to_string() +
    "F9BEB4D9" + "747800000000000000000000" + "02010000E293CDBE" +
    //"01000000016DBDDB085B1D8AF75184F0BC01FAD58D1266E9B63B50881990E4B40D6AEE3629000000008B483045022100F3581E1972AE8AC7C7367A7A253BC1135223ADB9A468BB3A59233F45BC578380022059AF01CA17D00E41837A1D58E97AA31BAE584EDEC28D35BD96923690913BAE9A0141049C02BFC97EF236CE6D8FE5D94013C721E915982ACD2B12B65D9B7D59E20A842005F8FC4E02532E873D37B96F09D6D4511ADA8F14042F46614A4C70C0F14BEFF5FFFFFFFF02404B4C00000000001976A9141AA0CD1CBEA6E7458A7ABAD512A9D9EA1AFB225E88AC80FAE9C7000000001976A9140EAB5BEA436A0484CFAB12485EFDA0B78B4ECC5288AC";
    "01000000016DBDDB085B1D8AF75184F0BC01FAD58D1266E9B63B50881990E4B40D6AEE3629000000008B483045022100F3581E1972AE8AC7C7367A7A253BC1135223ADB9A468BB3A59233F45BC578380022059AF01CA17D00E41837A1D58E97AA31BAE584EDEC28D35BD96923690913BAE9A0141049C02BFC97EF236CE6D8FE5D94013C721E915982ACD2B12B65D9B7D59E20A842005F8FC4E02532E873D37B96F09D6D4511ADA8F14042F46614A4C70C0F14BEFF5FFFFFFFF02404B4C00000000001976A9141AA0CD1CBEA6E7458A7ABAD512A9D9EA1AFB225E88AC80FAE9C7000000001976A9140EAB5BEA436A0484CFAB12485EFDA0B78B4ECC5288AC00000000";
  let msg_tx = btc::Msg::Msg::new_from_hex(&msg_tx_hex)
    .chain_err(|| "Falha no hex -> Msg no teste 1")?;
  println!("{:?}", msg_tx);

  let tx = if let Some(btc::Msg::Payload::Payload::Tx(tx)) = msg_tx.payload {
    Some(tx)
  } else {
    None
  };
  println!("{:?}", tx.unwrap());


  let msg_ping_hex = "".to_string() +
    "F9BEB4D9"+ "70696E670000000000000000" + "02010000E293CDBE" +
    "0094102111e2af4d";
  let msg_ping_vec: Vec<u8> = Vec::from_hex(msg_ping_hex)
    .chain_err(|| "Falha no hex -> Vec<u8>")?;
  let mut msg_ping_it = msg_ping_vec.into_iter();
  let msg_ping = btc::Msg::Msg::new(msg_ping_it.by_ref())
    .chain_err(|| "Falha no hex -> Msg no teste 2")?;
  println!("{:?\n}", msg_ping);


  let tx = if let Some(btc::Msg::Payload::Payload::Tx(tx)) = msg_ping.payload {
    Some(tx)
  } else {
    None
  };
  print!("{:?}\n\n", tx);


  let msg_pong_hex = "".to_string() +
    "F9BEB4D9"+ "706F6E670000000000000000" + "02010000E293CDBE" +
    "0194102111e2af4d";
  let msg_pong = btc::Msg::Msg::new_from_hex(&msg_pong_hex)
    .chain_err(|| "Falha no hex -> Msg no teste 3")?;
  println!("{:?}", msg_pong);


  let msg_version_hex = "".to_string() +
  "F9BEB4D976657273696F6E0000000000640000003B648D5A" +
  "62EA0000010000000000000011B2D05000000000010000000000000000000000000000000000FFFF000000000000010000000000000000000000000000000000FFFF0000000000003B2EB35D8CE617650F2F5361746F7368693A302E372E322FC03E0300";
  let msg_version = btc::Msg::Msg::new_from_hex(&msg_version_hex)
    .chain_err(|| "Falha no hex -> Msg no teste 4")?;
  println!("{:?}", msg_version);


  let msg_verack_hex = "F9BEB4D976657261636B000000000000000000005DF6E0E2";
  let msg_verack = btc::Msg::Msg::new_from_hex(&msg_verack_hex)
    .chain_err(|| "Falha no hex -> Msg no teste 5")?;
  println!("{:?}", msg_verack);


  Ok(())

}


fn main() {
  if let Err(ref e) = run() {
    use std::io::Write;
    let stderr = &mut ::std::io::stderr();
    let errmsg = "Error writing to stderr";
    writeln!(stderr, "error: {}", e).expect(errmsg);

    for e in e.iter().skip(1) {
      writeln!(stderr, "caused by: {}", e).expect(errmsg);
    }

    // The backtrace is not always generated. Try to run this example
    // with `RUST_BACKTRACE=1`.
    if let Some(backtrace) = e.backtrace() {
      writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
    }
    ::std::process::exit(1);
  }
}


