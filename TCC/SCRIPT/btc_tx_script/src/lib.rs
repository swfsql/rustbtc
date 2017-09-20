
//extern crate hex;
extern crate byteorder;
extern crate arrayvec;
use std::io::Cursor;
use std::fmt;
//use std::io::{Error, ErrorKind};
use std::error::Error;

//use hex::FromHex;
use byteorder::{LittleEndian, ReadBytesExt};
use arrayvec::ArrayVec;

pub struct Bytes(Vec<u8>);

/*
impl std::iter::Iterator for Bytes {
  type Item = u8;
  fn next(&mut self) -> Option<u8> {
      self.0.next()
    }
}
*/

use std::iter::Iterator;

impl std::iter::FromIterator<u8> for Bytes {
  fn from_iter<I: IntoIterator<Item=u8>>(iter: I) -> Self {
    let mut b = Bytes(Vec::new());
    for i in iter {
      b.0.push(i);
    }
    b
    // Bytes(iter.collect::<Vec<u8>>())
  }
}

impl std::fmt::Debug for Bytes {
  fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {

      let o = self.0.iter().enumerate()
        .map(|(i, s)|
        if i % 4 == 0 {
          if i % 8 == 0 {
            if i % 32 == 0 {
              if i == 0 {
                format!("\n│ ├{:>4},", s)
              } else {
                format!("\n│ │\n│ ├{:>4},", s)
              }
            } else {
              format!("\n│ ├{:>4},", s)
            }
          } else {
            format!("{:>5},", s)
          }
        } else {
          format!("{:>4},", s)
        })
        .collect::<String>() + "\n│ │";

      write!(f,"{}", o)
  }
}

pub struct Msg {
  pub header: MsgHeader,
  pub payload: Option<Box<std::fmt::Debug>>,
}

impl Msg {
  pub fn new(it: &mut std::vec::IntoIter<u8>) -> Msg {

    let header = MsgHeader::new(it);
    let payload = match header.cmd.clone().into_iter()
      .map(|x| x as char).collect::<String>().as_ref() {

      "tx" => Some(Box::new(Tx::new(it).unwrap())),
      "ping" => Some(Box::new(Ping::new(it).unwrap())),
      "pong" => Some(Box::new(Pong::new(it).unwrap())),
      _ => None,
    };

    // header.payload_len // TODO

    Msg {
      header: header,
      payload: payload,
    }
  }

}

/*
pub enum MsgPayload {
  Tx,
  Ping,
  Pong,
}*/

// https://en.bitcoin.it/wiki/Protocol_documentation#tx
pub struct MsgHeader {
  pub network: u32,
  pub cmd: ArrayVec<[u8; 12]>,
  pub payload_len: i32,
  pub payloadchk: u32,
}

impl MsgHeader {
  pub fn new(it: &mut std::vec::IntoIter<u8>) -> MsgHeader {
    MsgHeader {
      network: Cursor::new(it.take(4).collect::<Vec<u8>>())
        .read_u32::<LittleEndian>().unwrap(),
      cmd: it.take(12).map(|u| u.to_le()).collect::<ArrayVec<[u8; 12]>>(),
      payload_len: Cursor::new(it.take(4).collect::<Vec<u8>>())
        .read_i32::<LittleEndian>().unwrap(),
      payloadchk: Cursor::new(it.take(4).collect::<Vec<u8>>())
        .read_u32::<LittleEndian>().unwrap(),
    }
  }
}


impl std::fmt::Debug for MsgHeader {
  fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
      let mut s = "Message Header:\n".to_string();
      s += &format!("├ Message Network Identification: {}\n", self.network);
      s += &format!("├ Message Command OP_CODE: <{}>{:?}\n",
        self.cmd.clone().into_iter().map(|x| x as char).collect::<String>(),
        self.cmd.clone().into_iter().collect::<Bytes>());
      //str::from_utf8
      s += &format!("├ Payload Length: {}\n", self.payload_len);
      s += &format!("├ Payload Checksum: {}\n", self.payloadchk);

      write!(f, "{}", s)
  }
}


// https://bitcoin.org/en/developer-reference#ping
pub struct Ping {
  pub nounce: u64,
}

impl Ping {
  //pub fn new(it: &mut std::vec::IntoIter<u8>) -> Result<Ping, Box<Error>> {
  pub fn new(it: &mut std::vec::IntoIter<u8>) -> Box<std::dmt::Debug> {

    let nounce = Cursor::new(it.take(8).collect::<Vec<u8>>())
          .read_u64::<LittleEndian>()?;
    Ok(Ping {
      nounce: nounce,
    })
  }
}

impl std::fmt::Debug for Ping {
  fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
      let mut s = "Ping:\n".to_string();
      s += &format!("├ Nounce: {}\n", self.nounce);
      write!(f, "{}", s)
  }
}

// https://bitcoin.org/en/developer-reference#ping
pub struct Pong {
  pub nounce: u64,
}

impl Pong {
  //pub fn new(it: &mut std::vec::IntoIter<u8>) -> Result<Pong, Box<Error>> {
  pub fn new(it: &mut std::vec::IntoIter<u8>) -> Box<std::fmt::Debug> {

    let nounce = Cursor::new(it.take(8).collect::<Vec<u8>>())
          .read_u64::<LittleEndian>()?;
    Ok(Pong {
      nounce: nounce,
    })
  }
}

impl std::fmt::Debug for Pong {
  fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
      let mut s = "Pong:\n".to_string();
      s += &format!("├ Nounce: {}\n", self.nounce);
      write!(f, "{}", s)
  }
}

// https://en.bitcoin.it/wiki/Protocol_documentation#tx
// https://bitcoin.org/en/developer-reference#raw-transaction-format
pub struct Tx {
  pub version: i32,
  pub inputs_len: u8,
  pub inputs: Vec<TxInput>,
  pub outputs_len: u8,
  pub outputs: Vec<TxOutput>,
  pub locktime: u32,
  // TODO MAYBE witness
}

impl Tx {
  //pub fn new(it: &mut std::vec::IntoIter<u8>) -> Result<Tx, Box<Error>> {
  pub fn new(it: &mut std::vec::IntoIter<u8>) -> Box<std::fmt::Debug> {
    let ver = Cursor::new(it.by_ref().take(4).collect::<Vec<u8>>())
      .read_i32::<LittleEndian>()?;

    let ninputs = it.by_ref().next().ok_or("TODO")?.to_le();
    let mut inputs: Vec<TxInput> = vec![];
    for _ in 0..ninputs {
      inputs.push(TxInput::new(it));
    }

    let noutputs = it.by_ref().next().ok_or("TODO")?.to_le();
    let mut outputs: Vec<TxOutput> = vec![];
    for _ in 0..noutputs {
      outputs.push(TxOutput::new(it));
    }

    let locktime = Cursor::new(it.take(4).collect::<Vec<u8>>())
          .read_u32::<LittleEndian>()?;

    let tx = Tx {
      version: ver,
      inputs_len: ninputs,
      inputs: inputs,
      outputs_len: noutputs,
      outputs: outputs,
      locktime: locktime,
    };
    if let Some(_) = it.next() {
      Err("TODO")?;
    }
    Ok(tx)
  }
}


impl std::fmt::Debug for Tx {
  fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
      let mut s = "Tx:\n".to_string();
      s += &format!("├ Version: {}\n", self.version);
      s += &format!("├ Inputs Length: {}\n", self.inputs_len);
      s += &format!("├ Inputs:\n");
      for (i, input) in self.inputs.iter().enumerate() {
        s += &format!(" {:?}", input)
          .lines()
          .filter(|&x| x != "]")
          .enumerate()
          .map(|(i2, l)|
            if i2 == 0 {
              "│ ├".to_string() +
              &l.split(':').next().unwrap().to_string()
                .chars().collect::<String>() +
                &(i).to_string() + ":\n"
            } else {
              "│ │ ".to_string() + l + "\n"
            })
          .collect::<String>();
      }
      s += &format!("├ Outputs Length: {}\n", self.outputs_len);
      s += &format!("├ Outputs:\n");
      for (i, output) in self.outputs.iter().enumerate() {
        s += &format!(" {:?}", output)
          .lines()
          .filter(|&x| x != "]")
          .enumerate()
          .map(|(i2, l)|
            if i2 == 0 {
              "│ ├".to_string() +
              &l.split(':').next().unwrap().to_string()
                .chars().collect::<String>() +
                &(i).to_string() + ":\n"
            } else {
              "│ │ ".to_string() + l + "\n"
            })
          .collect::<String>();
      }
      //let inputs = format!(" {:?}", self.inputs)
      //s += &inputs;
      s += &format!("├ Locktime: {}\n", self.locktime);

      write!(f, "{}", s)
  }
}

pub struct TxInput {
  pub prev_tx: ArrayVec<[u8; 32]>,
  pub prev_tx_out_index: u32,
  pub script_len: u8,
  pub script_sig: Bytes,
  pub sequence: u32,
}

impl TxInput {
  pub fn new(it: &mut std::vec::IntoIter<u8>) -> TxInput {
  pub fn new(it: &mut std::vec::IntoIter<u8>) -> TxInput {
      let ptx = it.take(32).map(|u| u.to_le()).collect::<ArrayVec<[u8; 32]>>();
      let ptxoi = Cursor::new(it.take(4).collect::<Vec<u8>>())
          .read_u32::<LittleEndian>().unwrap();
      let slen = it.by_ref().next().unwrap().to_le();

      TxInput {
        prev_tx: ptx,
        prev_tx_out_index: ptxoi,
        script_len: slen,
        script_sig: it.take(slen as usize).map(|u| u.to_le())
          .collect::<Bytes>(),
        sequence: Cursor::new(it.take(4).collect::<Vec<u8>>())
          .read_u32::<LittleEndian>().unwrap(),
      }
  }
}


impl std::fmt::Debug for TxInput {
  fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
      let mut s = "Input:\n".to_string();
      s += &format!("├ Previous Tx: {:?}\n", self.prev_tx
        .clone().into_iter().collect::<Bytes>());
      s += &format!("├ Previous Tx Out Index: {}\n", self.prev_tx_out_index);
      s += &format!("├ Script Length: {}\n", self.script_len);
      s += &format!("├ Script Signature: {:?}\n", self.script_sig);
      s += &format!("├ Sequence: {}\n", self.sequence);

      write!(f, "{}", s)
  }
}


pub struct TxOutput {
  pub value: i64,
  pub pk_script_len: u8,
  pub pk_script: Bytes,
}

impl TxOutput {
  pub fn new(it: &mut std::vec::IntoIter<u8>) -> TxOutput {
      let val = Cursor::new(it.by_ref().take(8).collect::<Vec<u8>>())
        .read_i64::<LittleEndian>().unwrap();
      let pkslen = it.by_ref().next().unwrap().to_le();

      TxOutput {
        value: val,
        pk_script_len: pkslen,
        pk_script: it.take(pkslen as usize).map(|u| u.to_le())
          .collect::<Bytes>(),
      }
  }
}


impl std::fmt::Debug for TxOutput {
  fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
      let mut s = "Output:\n".to_string();
      s += &format!("├ Value: {}\n", self.value);
      s += &format!("├ PubKey Script Length: {}\n", self.pk_script_len);
      s += &format!("├ PubKey Script: {:?}\n", self.pk_script);

      write!(f,"{}", s)
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