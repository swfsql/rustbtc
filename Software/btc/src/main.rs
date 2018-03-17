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
enum ComposedStateMachina {
    #[state_machine_future(start, transitions(InnerB,InnerEnd))]
    InnerA(Peer),

    #[state_machine_future(transitions(InnerEnd))]
    InnerB(Peer),

    #[state_machine_future(ready)]
    InnerEnd((Peer, String)),

    #[state_machine_future(error)]
    InnerError(std::io::Error),
}

impl PollComposedStateMachina for ComposedStateMachina {
    fn poll_inner_a<'a>(
        peer: &'a mut RentToOwn<'a, InnerA>
    ) -> Poll<AfterInnerA, std::io::Error> {

        while let Some(msg) = try_ready!(peer.0.lines.poll()) {
            let msg = String::from_utf8(msg.to_vec()).unwrap();

            match msg.as_ref() {
                "B" => {
                    peer.0.lines.buffer("GOING TO B".as_bytes());
                    let _ = peer.0.lines.poll_flush()?;

                    let next = InnerB(peer.take().0);
                    println!("going to InnerB");
                    transition!(next)
                },
                _ =>  {
                    peer.0.lines.buffer("...".as_bytes());
                    let _ = peer.0.lines.poll_flush()?;

                    let next = InnerEnd((peer.take().0, msg));
                    println!("going to InnerEnd");
                    transition!(next)
                },
            }
        }
        // Err(std::io::Error::new(std::io::ErrorKind::ConnectionAborted, "Peer connection aborted."))
        panic!("Peer connection aborted.");
    }

    fn poll_inner_b<'a>(
        peer: &'a mut RentToOwn<'a, InnerB>
    ) -> Poll<AfterInnerB, std::io::Error> {

        while let Some(msg) = try_ready!(peer.0.lines.poll()) {
            let msg = String::from_utf8(msg.to_vec()).unwrap();

            let peer = peer.take();
            let next = InnerEnd((peer.0, msg));
            println!("going to InnerEnd");
            transition!(next)
        }
        // Err(std::io::Error::new(std::io::ErrorKind::ConnectionAborted, "Peer connection aborted."))
        panic!("Peer connection aborted.");
    }
}

#[derive(StateMachineFuture)]
enum Machina {
    #[state_machine_future(start, transitions(Standby))]
    Welcome(Peer),

    #[state_machine_future(transitions(Waiting))]
    Standby(Peer),

    #[state_machine_future(transitions(ComposedState, End))]
    Waiting(Peer),

    #[state_machine_future(transitions(Standby, Waiting))]
    ComposedState(ComposedStateMachinaFuture),

    #[state_machine_future(ready)]
    End(Peer),

    #[state_machine_future(error)]
    MachinaError(std::io::Error),
}

impl PollMachina for Machina {

    fn poll_welcome<'a>(
        peer: &'a mut RentToOwn<'a, Welcome>
    ) -> Poll<AfterWelcome, std::io::Error> {

        peer.0.lines.buffer("WELCOME".as_bytes());
        let _ = peer.0.lines.poll_flush()?;
        let _ = peer.0.lines.poll_flush()?; // to make sure
        println!("sent WELCOME");

        transition!(Standby(peer.take().0))
    }

    fn poll_standby<'a>(
        peer: &'a mut RentToOwn<'a, Standby>
    ) -> Poll<AfterStandby, std::io::Error> {

        while let Some(msg) = try_ready!(peer.0.lines.poll()) {
            let msg = String::from_utf8(msg.to_vec()).unwrap();

            match msg.as_ref() {
                "PING?" => {
                    println!("going to WAITING");
                    let peer = peer.take();
                    let waiting = Waiting(peer.0);
                    transition!(waiting)
                },
                _ => {
                    println!("BATATA: <{:?}>", &msg);
                },
            }
        }
        // Err(std::io::Error::new(std::io::ErrorKind::ConnectionAborted, "Peer connection aborted."))
        panic!("Peer connection aborted.");
    }

    fn poll_waiting<'a>(
        peer: &'a mut RentToOwn<'a, Waiting>
    ) -> Poll<AfterWaiting, std::io::Error> {

        while let Some(msg) = try_ready!(peer.0.lines.poll()) {
            let msg = String::from_utf8(msg.to_vec()).unwrap();

            match msg.as_ref() {
                "A" => {
                    peer.0.lines.buffer("Inside Composed State".as_bytes());
                    let _ = peer.0.lines.poll_flush()?;

                    let peer = peer.take();
                    let mach = ComposedStateMachina::start(peer.0);
                    let next = ComposedState(mach);
                    println!("going to ComposedState");
                    transition!(next)
                },
                "BYE" => {
                    peer.0.lines.buffer("HAVE A GOOD ONE".as_bytes());
                    let _ = peer.0.lines.poll_flush()?;

                    let peer = peer.take();
                    let next = End(peer.0);
                    println!("going to END");
                    transition!(next)
                },
                _ => {

                },
            }
        }
        // Err(std::io::Error::new(std::io::ErrorKind::ConnectionAborted, "Peer connection aborted."))
        panic!("Peer connection aborted.");
    }

    fn poll_composed_state<'a>(
        mach: &'a mut RentToOwn<'a, ComposedState>
    ) -> Poll<AfterComposedState, std::io::Error> {

        let (mut peer, mut msg) = try_ready!(mach.0.poll());

        match msg.as_ref() {
            "PING" => {
                peer.lines.buffer("PONG".as_bytes());
                let _ = peer.lines.poll_flush()?;

                let next = Standby(peer);
                println!("going to Standby");
                transition!(next)
            },
            _ => {
                peer.lines.buffer("...".as_bytes());
                let _ = peer.lines.poll_flush()?;

                let next = Waiting(peer);
                println!("going to Waiting");
                transition!(next)
            },
        }
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
