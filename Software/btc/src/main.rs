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



extern crate tokio;
extern crate futures;

use tokio::io;
use tokio::net::TcpListener;
use tokio::prelude::*;

use std::collections::HashMap;
use std::iter;
use std::env;
use std::io::{BufReader,BufWriter};
use std::sync::{Arc, Mutex};


struct Peer {
  socket: tokio::net::TcpStream,
  state: u32,
}

impl Peer {
  fn new(socket: tokio::net::TcpStream) -> Peer {
    Peer {
      socket,
      state: 0,
    }
  }

  fn task(socket: tokio::net::TcpStream) {

    // The client's socket address
    let addr = socket.peer_addr().unwrap();

    println!("New Connection: {}", addr);

    let (reader, writer) = socket.split();

    let reader = BufReader::new(reader);
    let writer = BufWriter::new(writer);

    let iter = stream::iter_ok::<_, io::Error>(iter::repeat(()));

    let socket_reader = iter.fold((reader, writer), move |(reader, writer), _| {
        // Read a line off the socket, failing if we're at EOF
        let line = io::read_until(reader, b'\n', Vec::new());
        let line = line.and_then(|(reader, vec)| {
            if vec.len() == 0 {
                Err(io::Error::new(io::ErrorKind::BrokenPipe, "broken pipe"))
            } else {
                Ok((reader, vec))
            }
        });

        // Convert the bytes we read into a string, and then send that
        // string to all other connected clients.
        let line = line.map(|(reader, vec)| {
            (reader, String::from_utf8(vec))
        });

        line.map(move |(reader, message)| {
            println!("{}: {:?}", addr, message);

            // state logic
            if let &Ok(ref msg) = &message {
              println!("CUSTOM MESSAGE: {:?}", msg);
              let msgback = format!("Take back this message: {}", &msg);
              let amt = io::write_all(writer, msgback.into_bytes());
            } else {
                // tx.unbounded_send("You didn't send valid UTF-8.".to_string()).unwrap();
            }

            (reader, writer)
        })
    });

    let socket_reader = socket_reader.map_err(|_| ());
    let connection = socket_reader.map(|_| ());

    // Spawn a task to process the connection
    tokio::spawn(connection.then(move |_| {
        println!("Connection {} closed.", addr);
        Ok(())
    }));
  } 
}

// impl Future for Peer {


// }


fn run() -> Result<()> {
  env_logger::init().unwrap();

  info!("\n\
    {}\n\
    -start-------------------", time::now().strftime("%Hh%Mm%Ss - D%d/M%m/Y%Y").unwrap());

    // Create the TCP listener we'll accept connections on.
    let addr = env::args().nth(1).unwrap_or("127.0.0.1:8080".to_string());
    let addr = addr.parse().unwrap();

    let listener = TcpListener::bind(&addr).unwrap();
    println!("Listening on: {}", addr);

    // The server task asynchronously iterates over and processes each incoming
    // connection.
    let srv = listener.incoming()
        .map_err(|e| println!("failed to accept socket; error = {:?}", e))
        .for_each(move |stream| {

            Peer::task(stream);



            Ok(())
        });

    // execute server
    tokio::run(srv);


  info!("\n\
    ---------------------end-\n\
    {}", time::now().strftime("%Hh%Mm%Ss - D%d/M%m/Y%Y").unwrap());
    Ok(())
}

quick_main!(run);
