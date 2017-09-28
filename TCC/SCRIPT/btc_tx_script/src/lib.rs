// TODO version
// TODO verack


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

extern crate hex;

use hex::FromHex;
use std::iter::Iterator;


pub struct Bytes(Vec<u8>);

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


pub trait NewFromHex {
  fn new_from_hex(hex: &str) -> Result<Self, Box<Error>>
  where Self: std::marker::Sized {
    let vec: Vec<u8> = Vec::from_hex(hex).unwrap();
    let mut it = vec.into_iter();
    Self::new(it.by_ref())
  }
  fn new(it: &mut std::vec::IntoIter<u8>) -> Result<Self, Box<Error>>
  where Self: std::marker::Sized;
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


pub enum MsgPayload {
  Tx(Tx),
  Ping(Ping),
  Pong(Pong),
  //Version(version),
  //Verack(verack),
}


pub struct Msg {
  pub header: MsgHeader,
  pub payload: Option<MsgPayload>,
}

impl NewFromHex for Msg {
  fn new(it: &mut std::vec::IntoIter<u8>) -> Result<Msg, Box<Error>> {

    let header = MsgHeader::new(it).unwrap();
    let cmd_str = header.cmd.clone().into_iter()
      .map(|x| x as char).collect::<String>();

    let payload = match cmd_str.to_string().trim().as_ref() {

      "tx\0\0\0\0\0\0\0\0\0\0" => Some(MsgPayload::Tx(Tx::new(it).unwrap())),
      "ping\0\0\0\0\0\0\0\0" => Some(MsgPayload::Ping(Ping::new(it).unwrap())),
      "pong\0\0\0\0\0\0\0\0" => Some(MsgPayload::Pong(Pong::new(it).unwrap())),
      _ => None,
    };

    // header.payload_len // TODO

    Ok(Msg {
      header: header,
      payload: payload,
    })
  }
}

impl std::fmt::Debug for Msg {
  fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
      let mut s = "Message:\n".to_string();
      s += &format!("├ Message Header: {:?}", self.header);
      s += &"├ Message Payload: \n".to_string();
      s += &match self.clone().payload {
          Some(ref p) => match p {
            &MsgPayload::Tx(ref tx) => format!("{:?}", tx),
            &MsgPayload::Ping(ref ping) => format!("{:?}", ping),
            &MsgPayload::Pong(ref pong) => format!("{:?}", pong),
          },
          None => "None".to_string(),
        }.lines().map(|x| "│ ".to_string() + x + "\n").collect::<String>();
      write!(f, "{}", s)
  }
}


// https://en.bitcoin.it/wiki/Protocol_documentation#tx
pub struct MsgHeader {
  pub network: u32,
  pub cmd: ArrayVec<[u8; 12]>,
  pub payload_len: i32,
  pub payloadchk: u32,
}

impl NewFromHex for MsgHeader {
  fn new(it: &mut std::vec::IntoIter<u8>) -> Result<MsgHeader, Box<Error>> {
    Ok(MsgHeader {
      network: Cursor::new(it.take(4).collect::<Vec<u8>>())
        .read_u32::<LittleEndian>().unwrap(),
      cmd: it.take(12).map(|u| u.to_le()).collect::<ArrayVec<[u8; 12]>>(),
      payload_len: Cursor::new(it.take(4).collect::<Vec<u8>>())
        .read_i32::<LittleEndian>().unwrap(),
      payloadchk: Cursor::new(it.take(4).collect::<Vec<u8>>())
        .read_u32::<LittleEndian>().unwrap(),
    })
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

impl NewFromHex for Ping {
  fn new(it: &mut std::vec::IntoIter<u8>) -> Result<Ping, Box<Error>> {
  //pub fn new(it: &mut std::vec::IntoIter<u8>) -> Result<Box<std::fmt::Debug>, Box<Error>> {

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

impl NewFromHex for Pong {
  fn new(it: &mut std::vec::IntoIter<u8>) -> Result<Pong, Box<Error>> {
  //pub fn new(it: &mut std::vec::IntoIter<u8>) -> Result<Box<std::fmt::Debug>, Box<Error>> {

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


enum VarUint {
  U8(u8),
  U16(u16),
  U32(u32),
  U64(u64),
}

impl VarUint {
  fn new(it: &mut std::vec::IntoIter<u8>) -> Result<VarUint, Box<Error>> {
    let value_head = it.by_ref().next().ok_or("TODO")?.to_le();
    match value_head {
      //0x00 .. 0xFC => VarInt::U8(value_head), // leu 1 byte
      0xFD => {
        let value_body = Cursor::new(it.take(2).collect::<Vec<u8>>())
          .read_u16::<LittleEndian>().unwrap();
        Ok(VarUint::U16(value_body))  // ler 16 bit
      },
      0xFE => { // ler 32 bit
        let value_body = Cursor::new(it.take(4).collect::<Vec<u8>>())
          .read_u32::<LittleEndian>().unwrap();
        Ok(VarUint::U32(value_body))
      },
      0xFF => { // ler 64 bit
        let value_body = Cursor::new(it.take(8).collect::<Vec<u8>>())
          .read_u64::<LittleEndian>().unwrap();
        Ok(VarUint::U64(value_body))
      },
      _ => Ok(VarUint::U8(value_head)), // leu 1 byte
    }
  }
}

struct VarStr {
  length: VarUint,
  string: Bytes,
}

impl VarStr {
  fn new(it: &mut std::vec::IntoIter<u8>) -> Result<VarStr, Box<Error>> {
    let len = VarUint::new(it).unwrap();
    let slen = match len {
      VarUint::U8(u) => Some(u as usize),
      VarUint::U16(u) => Some(u as usize),
      VarUint::U32(u) => Some(u as usize),
      VarUint::U64(u) => None, // u64 as usize is uncertain on x86 arch
    };

    Ok(VarStr {
      length: len,
      string: it.take(slen.unwrap()).map(|u| u.to_le()).collect::<Bytes>(),
    })
  }
}



// falta pub time: u32
// https://en.bitcoin.it/wiki/Protocol_documentation#Network_address
pub struct NetAddr {
  pub service: u64,
  pub ip: ArrayVec<[u8; 16]>,
  pub port: u16,
}

/*

// https://en.bitcoin.it/wiki/Protocol_documentation#version
// https://bitcoin.org/en/developer-reference#version
pub struct Version {
  pub version: i32,
  pub services: u64,
  pub timestamp: i64,
  pub addr_recv: NetAddr,
  pub addr_trans: NetAddr,
  pub nounce: u64,

  pub version: i32,

  pub version: i32,
  pub version: i32,




  pub version: i32,
  pub inputs_len: u8,
  pub inputs: Vec<TxInput>,
  pub outputs_len: u8,
  pub outputs: Vec<TxOutput>,
  pub locktime: u32,
  // TODO MAYBE witness
}

impl NewFromHex for Version {
  fn new(it: &mut std::vec::IntoIter<u8>) -> Result<Version, Box<Error>> {
  //pub fn new(it: &mut std::vec::IntoIter<u8>) -> Result<Box<std::fmt::Debug>, Box<Error>> {
    let ver = Cursor::new(it.by_ref().take(4).collect::<Vec<u8>>())
      .read_i32::<LittleEndian>()?;

    let ninputs = it.by_ref().next().ok_or("TODO")?.to_le();
    let mut inputs: Vec<TxInput> = vec![];
    for _ in 0..ninputs {
      inputs.push(TxInput::new(it).unwrap());
    }

    let noutputs = it.by_ref().next().ok_or("TODO")?.to_le();
    let mut outputs: Vec<TxOutput> = vec![];
    for _ in 0..noutputs {
      outputs.push(TxOutput::new(it).unwrap());
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

impl std::fmt::Debug for Version {
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

*/


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

impl NewFromHex for Tx {
  fn new(it: &mut std::vec::IntoIter<u8>) -> Result<Tx, Box<Error>> {
  //pub fn new(it: &mut std::vec::IntoIter<u8>) -> Result<Box<std::fmt::Debug>, Box<Error>> {
    let ver = Cursor::new(it.by_ref().take(4).collect::<Vec<u8>>())
      .read_i32::<LittleEndian>()?;

    let ninputs = it.by_ref().next().ok_or("TODO")?.to_le();
    let mut inputs: Vec<TxInput> = vec![];
    for _ in 0..ninputs {
      inputs.push(TxInput::new(it).unwrap());
    }

    let noutputs = it.by_ref().next().ok_or("TODO")?.to_le();
    let mut outputs: Vec<TxOutput> = vec![];
    for _ in 0..noutputs {
      outputs.push(TxOutput::new(it).unwrap());
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

impl NewFromHex for TxInput {
  fn new(it: &mut std::vec::IntoIter<u8>) -> Result<TxInput, Box<Error>> {
      let ptx = it.take(32).map(|u| u.to_le()).collect::<ArrayVec<[u8; 32]>>();
      let ptxoi = Cursor::new(it.take(4).collect::<Vec<u8>>())
          .read_u32::<LittleEndian>().unwrap();
      let slen = it.by_ref().next().unwrap().to_le();

      Ok(TxInput {
        prev_tx: ptx,
        prev_tx_out_index: ptxoi,
        script_len: slen,
        script_sig: it.take(slen as usize).map(|u| u.to_le())
          .collect::<Bytes>(),
        sequence: Cursor::new(it.take(4).collect::<Vec<u8>>())
          .read_u32::<LittleEndian>().unwrap(),
      })
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

impl NewFromHex for TxOutput {
  fn new(it: &mut std::vec::IntoIter<u8>) -> Result<TxOutput, Box<Error>> {
      let val = Cursor::new(it.by_ref().take(8).collect::<Vec<u8>>())
        .read_i64::<LittleEndian>().unwrap();
      let pkslen = it.by_ref().next().unwrap().to_le();

      Ok(TxOutput {
        value: val,
        pk_script_len: pkslen,
        pk_script: it.take(pkslen as usize).map(|u| u.to_le())
          .collect::<Bytes>(),
      })
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


// https://en.bitcoin.it/wiki/Protocol_documentation#tx

