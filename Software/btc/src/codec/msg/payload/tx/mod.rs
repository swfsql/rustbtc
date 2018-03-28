use std;
use std::fmt;
use codec::msg::commons::new_from_hex::NewFromHex;
use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};
mod errors {
    error_chain!{}
}
use errors::*;

pub mod input;
pub mod output;

// https://en.bitcoin.it/wiki/Protocol_documentation#tx
// https://bitcoin.org/en/developer-reference#raw-transaction-format
pub struct Tx {
    pub version: i32,
    pub inputs_len: u8,
    pub inputs: Vec<input::Input>,
    pub outputs_len: u8,
    pub outputs: Vec<output::Output>,
    pub locktime: u32,
    // TODO MAYBE witness
}

impl NewFromHex for Tx {
    fn new(it: &mut std::vec::IntoIter<u8>) -> Result<Tx> {
        //pub fn new(it: &mut std::vec::IntoIter<u8>) -> Result<Box<std::fmt::Debug>> {
        let aux = it.by_ref().take(4).collect::<Vec<u8>>();
        let version = Cursor::new(&aux).read_i32::<LittleEndian>().chain_err(|| {
            format!(
                "(Msg::payload::tx::Mod) Error at reading for version: read_i32 for {:?}",
                aux
            )
        })?;

        let inputs_len = it.by_ref()
            .next()
            .ok_or(
                "(Msg::payload::tx) Input feed ended unexpectedly when reading the input len info",
            )?
            .to_le();
        let mut inputs: Vec<input::Input> = vec![];
        for i in 0..inputs_len {
            let aux = input::Input::new(it).chain_err(|| {
                format!(
                    "(Msg::payload::tx::Mod)Error at creating a new input, at input {:?}",
                    i
                )
            })?;
            inputs.push(aux);
        }

        let outputs_len = it.by_ref()
            .next()
            .ok_or(
                "(Msg::payload::tx) Input feed ended unexpectedly when reading the output len info",
            )?
            .to_le();
        let mut outputs: Vec<output::Output> = vec![];
        for i in 0..outputs_len {
            let aux = output::Output::new(it).chain_err(|| {
                format!(
                    "(Msg::payload::tx::Mod)Error at creating a new Output, at outputs {}",
                    i
                )
            })?;
            outputs.push(aux);
        }

        let aux = it.take(4).collect::<Vec<u8>>();
        let locktime = Cursor::new(&aux).read_u32::<LittleEndian>().chain_err(|| {
            format!(
                "(Msg::payload::tx::Mod)Error at reading for locktime: read_u32 for value {:?}",
                aux
            )
        })?;

        let tx = Tx {
            version,
            inputs_len,
            inputs,
            outputs_len,
            outputs,
            locktime,
        };
        if it.next().is_some() {
            Err("(Msg::payload::tx::Mod)Error: input feed is bigger than expected")?;
        }
        Ok(tx)
    }
}

impl std::fmt::Debug for Tx {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        let mut s = "Tx:\n".to_string();
        s += &format!("├ Version: {}\n", self.version);
        s += &format!("├ Inputs Length: {}\n", self.inputs_len);
        s += "├ Inputs:\n";
        for (i, input) in self.inputs.iter().enumerate() {
            s += &format!(" {:?}", input)
                .lines()
                .filter(|&x| x != "]")
                .enumerate()
                .map(|(i2, l)| {
                    if i2 == 0 {
                        let aux = &l.split(':').next();
                        match *aux {
                            Some(a) => Ok("│ ├".to_string()
                                + &a.to_string().chars().collect::<String>()
                                + &(i).to_string()
                                + ":\n"),
                            None => Ok("ahh".to_string()), //Err(format!("Error when displaying input {}", l)),
                        }
                    } else {
                        Ok("│ │ ".to_string() + l + "\n")
                    }
                })
                .collect::<Result<String>>()
                .unwrap(); // TODO
                           //chain_err((|| "Error to display some input from a total of {}", self.inputs_len))?;
        }
        s += &format!("├ Outputs Length: {}\n", self.outputs_len);
        s += "├ Outputs:\n";
        for (i, output) in self.outputs.iter().enumerate() {
            s += &format!(" {:?}", output)
                .lines()
                .filter(|&x| x != "]")
                .enumerate()
                .map(|(i2, l)| {
                    if i2 == 0 {
                        "│ ├".to_string()
                            + &l.split(':').next().unwrap() // TODO
                .to_string()
                .chars().collect::<String>() + &(i).to_string() + ":\n"
                    } else {
                        "│ │ ".to_string() + l + "\n"
                    }
                })
                .collect::<String>();
        }
        //let inputs = format!(" {:?}", self.inputs)
        //s += &inputs;
        s += &format!("├ Locktime: {}\n", self.locktime);

        write!(f, "{}", s)
    }
}
