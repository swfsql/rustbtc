#![recursion_limit = "1024"]
#[macro_use]
extern crate error_chain;
mod errors {
    error_chain!{}
}
use errors::*;

#[macro_use] extern crate log;
extern crate env_logger;

extern crate hex;
extern crate time;

extern crate btc;

use btc::commons::new_from_hex::NewFromHex;
// use btc::commons::into_bytes::IntoBytes;

// usually ran with:
// RUST_LOG=btc=INFO cargo run

fn run() -> Result<()> {
  env_logger::init().unwrap();

  info!("\n\
    -------------------------\n\
    {}\n\
    -------------------------", time::now().strftime("%Hh%Mm%Ss - D%d/M%m/Y%Y").unwrap());

    {
        let msg_verack_hex = "F9BEB4D976657261636B000000000000000000005DF6E0E2";
        let msg_verack = btc::msg::Msg::new_from_hex(msg_verack_hex)
            .chain_err(|| "Falha no hex -> Msg no teste 5")?;
        println!("{:?}", msg_verack);
    }

    Ok(())
}

quick_main!(run);
