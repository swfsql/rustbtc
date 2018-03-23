#![recursion_limit = "1024"]
#[macro_use]
extern crate error_chain;
mod errors {
    error_chain!{}
}
use errors::*;

extern crate state_machine_future;

extern crate env_logger;
#[macro_use]
extern crate log;

extern crate hex;
extern crate time;

extern crate btc;

// use btc::commons::new_from_hex::NewFromHex;
// use btc::commons::into_bytes::IntoBytes;

// usually ran with:
// RUST_LOG=btc=INFO cargo run

extern crate tokio;

extern crate bytes;
extern crate futures;

#[macro_use]
extern crate structopt;

use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;

use std::thread;

fn process_peer(socket: TcpStream) {
    let peer = btc::peer::Peer::new(socket);

    //        .map_err(|_| ());

    let peer_machina = btc::peer::machina::Machina::start(peer)
        .map_err(|_| ())
        .map(|_| ());

    tokio::spawn(peer_machina);
    println!("depois do spawn");
}
fn process_admin(socket: TcpStream) {
    let peer = btc::admin::Peer::new(socket);

    //        .map_err(|_| ());

    let peer_machina = btc::admin::machina::Machina::start(peer)
        .map_err(|_| ())
        .map(|_| ());

    tokio::spawn(peer_machina);
    println!("depois do spawn");
}

fn run() -> Result<()> {
    env_logger::init().unwrap();

    info!(
        "\n\
         {}\n\
         -start-------------------",
        time::now().strftime("%Hh%Mm%Ss - D%d/M%m/Y%Y").unwrap()
    );

    let addr_peer = "127.0.0.1:8080".parse().unwrap();
    let addr_admin = "127.0.0.1:8081".parse().unwrap();

    let listener_peer = TcpListener::bind(&addr_peer).unwrap();
    let listener_admin = TcpListener::bind(&addr_admin).unwrap();

    let server_peer = listener_peer
        .incoming()
        .for_each(move |socket| {
            process_peer(socket);
            Ok(())
        })
        .map_err(|err| {
            println!("accept error = {:?}", err);
        });
    let server_admin = listener_admin
        .incoming()
        .for_each(move |socket| {
            process_admin(socket);
            Ok(())
        })
        .map_err(|err| {
            println!("accept error = {:?}", err);
        });

    println!("server_peer running on localhost:8080");
    println!("server_admin running on localhost:8081");

    thread::spawn(move || {
        tokio::run(server_admin);
    });

    tokio::run(server_peer);

    info!(
        "\n\
         ---------------------end-\n\
         {}",
        time::now().strftime("%Hh%Mm%Ss - D%d/M%m/Y%Y").unwrap()
    );
    Ok(())
}

quick_main!(run);
