// TODO compilar a caralha

#[macro_use]
extern crate error_chain;
mod errors {
    error_chain!{}
}

//extern crate hex;
extern crate arrayvec;
extern crate byteorder;

extern crate hex;


#[macro_use]
extern crate state_machine_future;
#[macro_use] extern crate log;
extern crate env_logger;
extern crate time;
extern crate tokio;
#[macro_use]
extern crate futures;
extern crate bytes;

pub mod msg;
pub mod commons;
pub mod peer;
pub mod codec;
