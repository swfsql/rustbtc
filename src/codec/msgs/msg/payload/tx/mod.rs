use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use codec::msgs::msg::commons::into_bytes::IntoBytes;
use codec::msgs::msg::commons::new_from_hex::NewFromHex;
use std;
use std::fmt;
use std::io::Cursor;
use std::iter::IntoIterator;

mod errors {
    error_chain!{}
}
use errors::*;

pub mod input;
pub mod output;

// https://en.bitcoin.it/wiki/Protocol_documentation#tx
// https://bitcoin.org/en/developer-reference#raw-transaction-format

#[derive(Clone)]
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
    fn new<'a, I>(it: I) -> Result<Tx>
    where
        I: IntoIterator<Item = &'a u8>,
    {
        let mut it = it.into_iter();
        //pub fn new<'a, I>(it: I) -> Result<Box<std::fmt::Debug>>
        let aux = it.by_ref().take(4).cloned().collect::<Vec<u8>>();
        let version = Cursor::new(&aux)
            .read_i32::<LittleEndian>()
            .chain_err(cf!("Error at reading for version: read_i32 for {:?}", aux))?;

        let inputs_len = it.next()
            .ok_or(ff!(
                "Input feed ended unexpectedly when reading the input len info"
            ))?
            .to_le();
        let mut inputs: Vec<input::Input> = vec![];
        for i in 0..inputs_len {
            let aux = input::Input::new(&mut it)
                .chain_err(cf!("Error at creating a new input, at input {:?}", i))?;
            inputs.push(aux);
        }

        let outputs_len = it.by_ref()
            .next()
            .ok_or(ff!(
                "Input feed ended unexpectedly when reading the output len info"
            ))?
            .to_le();
        let mut outputs: Vec<output::Output> = vec![];
        for i in 0..outputs_len {
            let aux = output::Output::new(it.by_ref())
                .chain_err(cf!("Error at creating a new Output, at outputs {}", i))?;
            outputs.push(aux);
        }

        let aux = it.by_ref().take(4).cloned().collect::<Vec<u8>>();
        let locktime = Cursor::new(&aux).read_u32::<LittleEndian>().chain_err(cf!(
            "Error at reading for locktime: read_u32 for value {:?}",
            aux
        ))?;

        let tx = Tx {
            version,
            inputs_len,
            inputs,
            outputs_len,
            outputs,
            locktime,
        };
        if it.next().is_some() {
            Err(ff!("Error: input feed is bigger than expected"))?;
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
                .expect(&ff!(
                    "Error to display some input from a total of {}",
                    self.inputs_len
                ));
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
                            + &l.split(':')
                                .next()
                                .expect(&ff!())
                                .to_string()
                                .chars()
                                .collect::<String>() + &(i).to_string()
                            + ":\n"
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

impl IntoBytes for Tx {
    fn into_bytes(&self) -> Result<Vec<u8>> {
        let mut wtr = vec![];
        wtr.write_i32::<LittleEndian>(self.version).chain_err(cf!(
            "Failure to convert version ({}) into byte vec",
            self.version
        ))?;
        wtr.write_u8(self.inputs_len).chain_err(cf!(
            "Failure to convert inputs_len ({}) into byte vec",
            self.inputs_len
        ))?;
        let inputs = self.inputs
            .iter()
            .map(|input| input.into_bytes())
            .collect::<Result<Vec<_>>>()?;
        for mut input in inputs {
            wtr.append(&mut input);
        }
        wtr.write_u8(self.outputs_len).chain_err(cf!(
            "Failure to convert outputs_len ({}) into byte vec",
            self.outputs_len
        ))?;
        let outputs = self.outputs
            .iter()
            .map(|output| output.into_bytes())
            .collect::<Result<Vec<_>>>()?;
        for mut output in outputs {
            wtr.append(&mut output);
        }
        wtr.write_u32::<LittleEndian>(self.locktime).chain_err(cf!(
            "Failure to convert locktime ({}) into byte vec",
            self.locktime
        ))?;
        Ok(wtr)
    }
}
