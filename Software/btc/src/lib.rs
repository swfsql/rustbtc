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

extern crate bytes;
extern crate env_logger;
#[macro_use]
extern crate futures;
extern crate log;
#[macro_use]
extern crate state_machine_future;
extern crate time;
extern crate tokio;

#[macro_use]
extern crate structopt;

pub mod peer;
pub mod admin;
pub mod codec;
pub mod scheduler;
