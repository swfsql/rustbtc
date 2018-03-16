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

//@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@//

struct Peer {
    lines: Lines,
}

#[derive(Debug)]
struct Lines {
    socket: TcpStream,
    rd: BytesMut,
    wr: BytesMut,
}

impl Peer {
    fn new(lines: Lines) -> Peer
    {
        let addr = lines.socket.peer_addr().unwrap();

        Peer {
            lines,
        }
    }
}

impl Future for Peer {
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<(), io::Error> {

        println!("poll called");

        let _ = self.lines.poll_flush()?;

        while let Async::Ready(line) = self.lines.poll()? {
            println!("Received line : {:?}", line);

            if let Some(message) = line {

                let mut line = message.clone();
                line.put("\r\n");

                let line = line.freeze();
                //self.msgs_to_send.push(line.clone());
                self.lines.buffer(&line.clone());

            } else {
                return Ok(Async::Ready(()));
            }
        }


        let _ = self.lines.poll_flush()?;

        Ok(Async::NotReady)
    }
}

impl Lines {
    fn new(socket: TcpStream) -> Self {
        Lines {
            socket,
            rd: BytesMut::new(),
            wr: BytesMut::new(),
        }
    }

    fn buffer(&mut self, line: &[u8]) {
        self.wr.reserve(line.len());

        self.wr.put(line);
    }

    fn poll_flush(&mut self) -> Poll<(), io::Error> {
        while !self.wr.is_empty() {
            let n = try_ready!(self.socket.poll_write(&self.wr));

            assert!(n > 0);

            let _ = self.wr.split_to(n);
        }

        Ok(Async::Ready(()))
    }

    fn fill_read_buf(&mut self) -> Poll<(), io::Error> {
        loop {
            self.rd.reserve(1024);

            let n = try_ready!(self.socket.read_buf(&mut self.rd));

            if n == 0 {
                return Ok(Async::Ready(()));
            }
        }
    }
}

impl Stream for Lines {
    type Item = BytesMut;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {

        println!("lines poll called");

        let sock_closed = self.fill_read_buf()?.is_ready();

        let pos = self.rd.windows(2).enumerate()
            .find(|&(_, bytes)| bytes == b"\r\n")
            .map(|(i, _)| i);

        if let Some(pos) = pos {
            let mut line = self.rd.split_to(pos + 2);

            line.split_off(pos);

            return Ok(Async::Ready(Some(line)));
        }

        if sock_closed {
            Ok(Async::Ready(None))
        } else {
            Ok(Async::NotReady)
        }
    }
}


#[derive(StateMachineFuture)]
enum Machina {
    #[state_machine_future(start, transitions(Waiting))]
    Init(Peer),

    #[state_machine_future(transitions(Init, End))]
    Waiting(Peer),

    #[state_machine_future(ready)]
    End(Peer),

    #[state_machine_future(error)]
    Error(std::io::Error),
}

impl PollMachina for Machina {
    fn poll_init<'a>(
        peer: &'a mut RentToOwn<'a, Init>
    ) -> Poll<AfterInit, std::io::Error> {

        while let Some(msg) = try_ready!(peer.0.lines.poll()) {
            let msg = String::from_utf8(msg.to_vec()).unwrap();

            if msg == "PING?" {
                let peer = peer.take();
                let waiting = Waiting(peer.0);
                transition!(waiting)
            }
        }
        panic!("peer machina invalid state")
    }

    fn poll_waiting<'a>(
        peer: &'a mut RentToOwn<'a, Waiting>
    ) -> Poll<AfterWaiting, std::io::Error> {

        while let Some(msg) = try_ready!(peer.0.lines.poll()) {
            let msg = String::from_utf8(msg.to_vec()).unwrap();

            if msg == "PING" {
                peer.0.lines.buffer("PONG".as_bytes());
                let _ = peer.0.lines.poll_flush()?;

                let peer = peer.take();
                let init = Init(peer.0);
                transition!(init)

            } else if msg == "BYE" {
                peer.0.lines.buffer("HAVE A GOOD ONE".as_bytes());
                let _ = peer.0.lines.poll_flush()?;

                let peer = peer.take();
                let end = End(peer.0);
                transition!(end)
            }
        }
        panic!("peer machina invalid state")
    }
}



fn process(socket: TcpStream) {

    let peer = Peer::new(
            Lines::new(socket)
        );

//        .map_err(|_| ());

    let peer_machina = Machina::start(peer)
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


    let addr = "127.0.0.1:6142".parse().unwrap();

    let listener = TcpListener::bind(&addr).unwrap();

    let server = listener.incoming().for_each(move |socket| {
        process(socket);
        Ok(())
    })
    .map_err(|err| {
        println!("accept error = {:?}", err);
    });

    println!("server running on localhost:6142");
    tokio::run(server);


  info!("\n\
    ---------------------end-\n\
    {}", time::now().strftime("%Hh%Mm%Ss - D%d/M%m/Y%Y").unwrap());
    Ok(())
}

quick_main!(run);
