use tokio::io;
use tokio::net::TcpStream;
use tokio::prelude::*;
use futures::{Async, Future, Poll};
use bytes::BufMut;
use codec::lines::Lines;
use futures::sync::{mpsc, oneshot};

use scheduler::commons::{AddrReqId, RequestId, Rx_mpsc_sf, Rx_one, Tx_mpsc,
                    Tx_one, WorkerRequestContent,
                    WorkerResponseContent, Rx_peers};

pub mod machina;
pub mod args;

pub struct Peer {
    lines: Lines,
    tx_req: mpsc::Sender<Box<WorkerRequestContent>>,
}

impl Peer {
    pub fn new(socket: TcpStream, tx_req: mpsc::Sender<Box<WorkerRequestContent>>) -> Peer {
        // let addr = lines.socket.peer_addr().unwrap();

        Peer {
            lines: Lines::new(socket),
            tx_req: tx_req,
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
