#![recursion_limit = "1024"]
#[macro_use]
extern crate error_chain;
mod errors {
    error_chain!{}
}
use errors::*;

#[macro_use]
extern crate state_machine_future;


#[macro_use] extern crate log;
extern crate env_logger;

extern crate hex;
extern crate time;

extern crate btc;

// use btc::commons::new_from_hex::NewFromHex;
// use btc::commons::into_bytes::IntoBytes;

// usually ran with:
// RUST_LOG=btc=INFO cargo run


extern crate tokio;

#[macro_use]
extern crate futures;
extern crate bytes;

use tokio::io;
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;
use futures::{Async, Future, Poll };
use futures::future::{self, Either};
use bytes::{BytesMut, Bytes, BufMut};

use state_machine_future::RentToOwn;

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex,mpsc};




fn process(socket: TcpStream) {

    let peer = btc::peer::Peer::new(socket); 

//        .map_err(|_| ());

    let peer_machina = btc::peer::machina::Machina::start(peer)
        .map_err(|_| ())
        .map(|_| ());

    tokio::spawn(peer_machina);
    println!("depois do spawn");
}


fn run() -> Result<()> {
  env_logger::init().unwrap();

  info!("\n\
    {}\n\
    -start-------------------", time::now().strftime("%Hh%Mm%Ss - D%d/M%m/Y%Y").unwrap());


    let addr = "127.0.0.1:8080".parse().unwrap();

    let listener = TcpListener::bind(&addr).unwrap();

    let server = listener.incoming().for_each(move |socket| {
        process(socket);
        Ok(())
    })
    .map_err(|err| {
        println!("accept error = {:?}", err);
    });

    println!("server running on localhost:8080");
    tokio::run(server);


  info!("\n\
    ---------------------end-\n\
    {}", time::now().strftime("%Hh%Mm%Ss - D%d/M%m/Y%Y").unwrap());
    Ok(())
}

quick_main!(run);
