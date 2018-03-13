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

// use btc::commons::new_from_hex::NewFromHex;
// use btc::commons::into_bytes::IntoBytes;

// usually ran with:
// RUST_LOG=btc=INFO cargo run

fn run() -> Result<()> {
  env_logger::init().unwrap();

  info!("\n\
    {}\n\
    -start-------------------", time::now().strftime("%Hh%Mm%Ss - D%d/M%m/Y%Y").unwrap());


  info!("\n\
    ---------------------end-\n\
    {}", time::now().strftime("%Hh%Mm%Ss - D%d/M%m/Y%Y").unwrap());
    Ok(())
}

quick_main!(run);
