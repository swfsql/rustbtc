use std;
use std::fmt;
use Commons::NewFromHex::NewFromHex;
use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};
mod errors {
    error_chain!{}
}
use errors::*;

pub mod Input;
pub mod Output;

// https://en.bitcoin.it/wiki/Protocol_documentation#tx
// https://bitcoin.org/en/developer-reference#raw-transaction-format
pub struct Tx {
  pub version: i32,
  pub inputs_len: u8,
  pub inputs: Vec<Input::Input>,
  pub outputs_len: u8,
  pub outputs: Vec<Output::Output>,
  pub locktime: u32,
  // TODO MAYBE witness
}

impl NewFromHex for Tx {
  fn new(it: &mut std::vec::IntoIter<u8>) -> Result<Tx> {
  //pub fn new(it: &mut std::vec::IntoIter<u8>) -> Result<Box<std::fmt::Debug>> {
    let ver = Cursor::new(it.by_ref().take(4).collect::<Vec<u8>>())
      .read_i32::<LittleEndian>().chain_err(|| "")?;

    let ninputs = it.by_ref().next().ok_or("TODO")?.to_le();
    let mut inputs: Vec<Input::Input> = vec![];
    for _ in 0..ninputs {
      inputs.push(Input::Input::new(it).unwrap());
    }

    let noutputs = it.by_ref().next().ok_or("TODO")?.to_le();
    let mut outputs: Vec<Output::Output> = vec![];
    for _ in 0..noutputs {
      outputs.push(Output::Output::new(it).unwrap());
    }

    let locktime = Cursor::new(it.take(4).collect::<Vec<u8>>())
          .read_u32::<LittleEndian>().chain_err(|| "")?;

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

