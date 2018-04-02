#![feature(box_patterns)]


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
#[macro_use]
extern crate log;
#[macro_use]
extern crate state_machine_future;
extern crate time;
extern crate tokio;
extern crate tokio_timer;

#[macro_use]
extern crate structopt;

#[macro_use]
pub mod macros;

pub mod peer;
pub mod admin;
pub mod codec;
pub mod exec;


/*
#[macro_use]
pub mod macros;

*/

/*
macro_rules! e {
    () => (error!("{} {} {}", module_path!(), file!(), line!()));
    ($fmt:expr) => (error!("{} {} {} \n {}", module_path!(), file!(), line!(), $fmt));
    ($fmt:expr, $($arg:tt)*) => (error!("{} {} {} \n {}", module_path!(), file!(), line!(), $fmt, $($arg)*));
}
*/