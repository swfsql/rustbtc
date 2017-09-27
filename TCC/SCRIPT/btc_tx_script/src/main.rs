
/*

TODO: validar Msg de acordo com payload_len
TALVEZ: Validar de acordo com o checksum
TODO: melhorar Debug for Msg
*/

extern crate hex;
use hex::FromHex;
use btc_tx_script::NewFromHex;

extern crate btc_tx_script;
use btc_tx_script as btctx;

fn main() {
  let msg_tx_hex = "".to_string() +
    "F9BEB4D9" + "747800000000000000000000" + "02010000E293CDBE" +
    "01000000016DBDDB085B1D8AF75184F0BC01FAD58D1266E9B63B50881990E4B40D6AEE3629000000008B483045022100F3581E1972AE8AC7C7367A7A253BC1135223ADB9A468BB3A59233F45BC578380022059AF01CA17D00E41837A1D58E97AA31BAE584EDEC28D35BD96923690913BAE9A0141049C02BFC97EF236CE6D8FE5D94013C721E915982ACD2B12B65D9B7D59E20A842005F8FC4E02532E873D37B96F09D6D4511ADA8F14042F46614A4C70C0F14BEFF5FFFFFFFF02404B4C00000000001976A9141AA0CD1CBEA6E7458A7ABAD512A9D9EA1AFB225E88AC80FAE9C7000000001976A9140EAB5BEA436A0484CFAB12485EFDA0B78B4ECC5288AC00000000";
  let msg_tx = btctx::Msg::new_from_hex(&msg_tx_hex).unwrap();
  println!("{:?}", msg_tx);

  let tx = if let Some(btctx::MsgPayload::Tx(tx)) = msg_tx.payload {
    Some(tx)
  } else {
    None
  };
  println!("{:?}", tx.unwrap());



  //ping
  let msg_ping_hex = "".to_string() +
    "F9BEB4D9"+ "70696E670000000000000000" + "02010000E293CDBE" +
    "0094102111e2af4d";
  let msg_ping_vec: Vec<u8> = Vec::from_hex(msg_ping_hex).unwrap();
  let mut msg_ping_it = msg_ping_vec.into_iter();
  let msg_ping = btctx::Msg::new(msg_ping_it.by_ref()).unwrap();
  println!("{:?\n}", msg_ping);


  let tx = if let Some(btctx::MsgPayload::Tx(tx)) = msg_ping.payload {
    Some(tx)
  } else {
    None
  };
  print!("{:?}\n\n", tx);

  //pong

  let msg_pong_hex = "".to_string() +
    "F9BEB4D9"+ "706F6E670000000000000000" + "02010000E293CDBE" +
    "0194102111e2af4d";
  let msg_pong = btctx::Msg::new_from_hex(&msg_pong_hex).unwrap();
  println!("{:?}", msg_pong);


  // example tx msg
  // https://en.bitcoin.it/wiki/Protocol_documentation#tx
  //let tx_msg = "F9BEB4D974780000000000000000000002010000E293CDBE01000000016DBDDB085B1D8AF75184F0BC01FAD58D1266E9B63B50881990E4B40D6AEE3629000000008B483045022100F3581E1972AE8AC7C7367A7A253BC1135223ADB9A468BB3A59233F45BC578380022059AF01CA17D00E41837A1D58E97AA31BAE584EDEC28D35BD96923690913BAE9A0141049C02BFC97EF236CE6D8FE5D94013C721E915982ACD2B12B65D9B7D59E20A842005F8FC4E02532E873D37B96F09D6D4511ADA8F14042F46614A4C70C0F14BEFF5FFFFFFFF02404B4C00000000001976A9141AA0CD1CBEA6E7458A7ABAD512A9D9EA1AFB225E88AC80FAE9C7000000001976A9140EAB5BEA436A0484CFAB12485EFDA0B78B4ECC5288AC00000000";
  //let vec_msg: Vec<u8> = Vec::from_hex(tx_msg).unwrap();
  //let mut it = vec_msg.into_iter();
  //let header = btctx::MsgHeader::new(it.by_ref());
  //println!("\n{:?}", header);
  //let tx = btctx::Tx::new(it.by_ref()).unwrap();
  //println!("{:?}", tx);

  // https://bitcoin.org/en/developer-examples#simple-raw-transaction
  //let tx_raw_hex = "01000000017b1eabe0209b1fe794124575ef807057c77ada2138ae4fa8d6c4de0398a14f3f0000000000ffffffff01f0ca052a010000001976a914cbc20a7664f2f69e5355aa427045bc15e7c6c77288ac00000000";
  // ...

  // https://bitcoin.org/en/developer-examples#complex-raw-transaction
  //let tx_raw_hex = "0100000002f327e86da3e66bd20e1129b1fb36d07056f0b9a117199e759396526b8f3a20780000000000fffffffff0ede03d75050f20801d50358829ae02c058e8677d2cc74df51f738285013c260000000000ffffffff02f028d6dc010000001976a914ffb035781c3c69e076d48b60c3d38592e7ce06a788ac00ca9a3b000000001976a914fa5139067622fd7e1e722a05c17c2bb7d5fd6df088ac00000000";
  //let tx_raw_hex_vec_msg: Vec<u8> = Vec::from_hex(tx_raw_hex).unwrap();
  //let mut it2 = tx_raw_hex_vec_msg.into_iter();
  //let tx2 = btctx::Tx::new(it2.by_ref()).unwrap();
  //println!("{:?}", tx2);

  //0094102111e2af4d

  //let pl_ping_hex = "0094102111e2af4d";
  //let pl_ping_hex_vec: Vec<u8> = Vec::from_hex(pl_ping_hex).unwrap();

  //let mut pl_ping_it = pl_ping_hex_vec.into_iter();
  //let pl_ping = btctx::Ping::new(pl_ping_it.by_ref()).unwrap();
  //println!("{:?}", pl_ping);


}

