#![feature(box_patterns)]
#![feature(exclusive_range_pattern)]

#[macro_use]
extern crate error_chain;
mod errors {
    error_chain!{}
}

//extern crate hex;
extern crate arrayvec;
extern crate byteorder;

extern crate bytes;
extern crate env_logger;
extern crate hex;
#[macro_use]
extern crate futures;
#[macro_use]
extern crate log;
#[macro_use]
extern crate state_machine_future;
extern crate chrono;
extern crate rand;
extern crate time;
extern crate tokio;
extern crate tokio_timer;

#[macro_use]
extern crate structopt;

#[macro_use]
pub mod macros;

#[macro_use]
extern crate defmac;

pub mod admin;
pub mod codec;
pub mod exec;
pub mod peer;