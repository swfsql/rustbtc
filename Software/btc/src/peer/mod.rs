use tokio::io;
use tokio::net::TcpStream;
use tokio::prelude::*;
use futures::{Async, Future, Poll};
use bytes::BufMut;

use codec::lines::Lines;

pub mod machina;

pub struct Peer {
    lines: Lines,

}

impl Peer {
    pub fn new(socket: TcpStream) -> Peer {
        // let addr = lines.socket.peer_addr().unwrap();

        Peer {
            lines: Lines::new(socket),
        }
    }
}

impl Future for Peer {
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<(), io::Error> {
        d!("poll called");

        let _ = self.lines.poll_flush()?;

        while let Async::Ready(line) = self.lines.poll()? {
            i!("Received line : {:?}", line);

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
