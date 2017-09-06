extern crate hex;
extern crate byteorder;
extern crate arrayvec;
use std::io::Cursor;

use hex::FromHex;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use arrayvec::ArrayVec;

fn main() {

    // example tx msg
    let tx_msg = "F9BEB4D974780000000000000000000002010000E293CDBE01000000016DBDDB085B1D8AF75184F0BC01FAD58D1266E9B63B50881990E4B40D6AEE3629000000008B483045022100F3581E1972AE8AC7C7367A7A253BC1135223ADB9A468BB3A59233F45BC578380022059AF01CA17D00E41837A1D58E97AA31BAE584EDEC28D35BD96923690913BAE9A0141049C02BFC97EF236CE6D8FE5D94013C721E915982ACD2B12B65D9B7D59E20A842005F8FC4E02532E873D37B96F09D6D4511ADA8F14042F46614A4C70C0F14BEFF5FFFFFFFF02404B4C00000000001976A9141AA0CD1CBEA6E7458A7ABAD512A9D9EA1AFB225E88AC80FAE9C7000000001976A9140EAB5BEA436A0484CFAB12485EFDA0B78B4ECC5288AC00000000";
    let vec_msg: Vec<u8> = Vec::from_hex(tx_msg).unwrap();

    for u in &vec_msg {
      print!("{}-", u);
    }
    println!("");


  let mut it = vec_msg.into_iter();
  let header = MsgHeader::new(it.by_ref());


    println!("\nMessage header:\n");
    println!("Message network identification: {}", header.network);
    println!("Message command OP_CODE: {:?}", header.cmd);
    println!("Payload: {}", header.payload);
    println!("Payload Checksum: {}", header.payloadchk);

    let tx = Tx::new(it.by_ref());
    println!("\nTransaction :\n{:?}", tx);

}
/*
struct Input {
  outpoint:
}
*/
// ArrayVec<[_; 3]>
// ArrayVec::<[_; 16]>::new();

#[derive(Debug)]
struct MsgHeader {
  network: u32,
  cmd: ArrayVec<[u8; 12]>,
  payload: i32,
  payloadchk: u32,
}

impl MsgHeader {
  fn new(it: &mut std::vec::IntoIter<u8>) -> MsgHeader {
    MsgHeader {
      network: Cursor::new(it.take(4).collect::<Vec<u8>>())
        .read_u32::<LittleEndian>().unwrap(),
      cmd: it.take(12).map(|u| u.to_le()).collect::<ArrayVec<[u8; 12]>>(),
      payload: Cursor::new(it.take(4).collect::<Vec<u8>>())
        .read_i32::<LittleEndian>().unwrap(),
      payloadchk: Cursor::new(it.take(4).collect::<Vec<u8>>())
        .read_u32::<LittleEndian>().unwrap(),
    }
  }
}

#[derive(Debug)]
struct Tx {
  version: i32,
  inputsLen: u8,
  inputs: Vec<TxInput>,
  outputsLen: u8,
  outputs: Vec<TxOutput>,
  locktime: u32,
  // TODO MAYBE witness
}

impl Tx {
  fn new(it: &mut std::vec::IntoIter<u8>) -> Tx {
    let ver = Cursor::new(it.by_ref().take(4).collect::<Vec<u8>>())
      .read_i32::<LittleEndian>().unwrap();

    let ninputs = it.by_ref().next().unwrap().to_le();
    let mut inputs: Vec<TxInput> = vec![];
    for i in (0..ninputs) {
      inputs.push(TxInput::new(it));
    }

    let noutputs = it.by_ref().next().unwrap().to_le();
    let mut outputs: Vec<TxOutput> = vec![];
    for i in (0..noutputs) {
      outputs.push(TxOutput::new(it));
    }

    Tx {
      version: ver,
      inputsLen: ninputs,
      inputs: inputs,
      outputsLen: noutputs,
      outputs: outputs,
      locktime: Cursor::new(it.take(4).collect::<Vec<u8>>())
          .read_u32::<LittleEndian>().unwrap(),
    }

  }
}

#[derive(Debug)]
struct TxInput {
  prevTx: ArrayVec<[u8; 32]>,
  prevTxOutIndex: u32,
  scriptLen: u8,
  scriptSig: Vec<u8>,
  sequence: u32,
}

impl TxInput {
  fn new(it: &mut std::vec::IntoIter<u8>) -> TxInput {
      let ptx = it.take(32).map(|u| u.to_le()).collect::<ArrayVec<[u8; 32]>>();
      let ptxoi = Cursor::new(it.take(4).collect::<Vec<u8>>())
          .read_u32::<LittleEndian>().unwrap();
      let slen = it.by_ref().next().unwrap().to_le();

      TxInput {
        prevTx: ptx,
        prevTxOutIndex: ptxoi,
        scriptLen: slen,
        scriptSig: it.take(slen as usize).map(|u| u.to_le()).collect::<Vec<u8>>(),
        sequence: Cursor::new(it.take(4).collect::<Vec<u8>>())
          .read_u32::<LittleEndian>().unwrap(),
      }
  }
}

#[derive(Debug)]
struct TxOutput {
  value: i64,
  pkScriptLen: u8,
  pkScript: Vec<u8>,
}

impl TxOutput {
  fn new(it: &mut std::vec::IntoIter<u8>) -> TxOutput {
      let val = Cursor::new(it.by_ref().take(8).collect::<Vec<u8>>())
        .read_i64::<LittleEndian>().unwrap();
      let pkslen = it.by_ref().next().unwrap().to_le();

      TxOutput {
        value: val,
        pkScriptLen: pkslen,
        pkScript: it.take(pkslen as usize).map(|u| u.to_le()).collect::<Vec<u8>>(),
      }
  }
}


/*
scriptSig:
  <sig> <pubKey>
*/

/*
// https://en.bitcoin.it/wiki/Protocol_documentation#tx

// example tx msg
F9 BE B4 D9 74 78 00 00  00 00 00 00 00 00 00 00
02 01 00 00 E2 93 CD BE  01 00 00 00 01 6D BD DB
08 5B 1D 8A F7 51 84 F0  BC 01 FA D5 8D 12 66 E9
B6 3B 50 88 19 90 E4 B4  0D 6A EE 36 29 00 00 00
00 8B 48 30 45 02 21 00  F3 58 1E 19 72 AE 8A C7
C7 36 7A 7A 25 3B C1 13  52 23 AD B9 A4 68 BB 3A
59 23 3F 45 BC 57 83 80  02 20 59 AF 01 CA 17 D0
0E 41 83 7A 1D 58 E9 7A  A3 1B AE 58 4E DE C2 8D
35 BD 96 92 36 90 91 3B  AE 9A 01 41 04 9C 02 BF
C9 7E F2 36 CE 6D 8F E5  D9 40 13 C7 21 E9 15 98
2A CD 2B 12 B6 5D 9B 7D  59 E2 0A 84 20 05 F8 FC
4E 02 53 2E 87 3D 37 B9  6F 09 D6 D4 51 1A DA 8F
14 04 2F 46 61 4A 4C 70  C0 F1 4B EF F5 FF FF FF
FF 02 40 4B 4C 00 00 00  00 00 19 76 A9 14 1A A0
CD 1C BE A6 E7 45 8A 7A  BA D5 12 A9 D9 EA 1A FB
22 5E 88 AC 80 FA E9 C7  00 00 00 00 19 76 A9 14
0E AB 5B EA 43 6A 04 84  CF AB 12 48 5E FD A0 B7
8B 4E CC 52 88 AC 00 00  00 00

77
0000 0000

*/

// Almost all integers are encoded in little endian. Only IP or port number are encoded big endian.


// ARRAYVEC