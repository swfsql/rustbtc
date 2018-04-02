use std::sync::{Arc, Mutex};
use tokio::io;
use tokio::net::TcpStream;
use tokio::prelude::*;
use futures::{Async, Future, Poll};
use bytes::BufMut;
use codec::lines::Lines;
//use futures::sync::{mpsc, oneshot};
use futures::sync::{mpsc};

//use exec::commons::{AddrReqId, RequestId, RxMpscSf, RxOne, TxMpsc,
//                    TxOne, WorkerRequestContent,
//                    WorkerResponseContent, RxPeers};

use exec::commons::{WorkerRequestContent,RxPeers,WorkerToPeerRequestAndPriority,ToolBox,TxMpscMainToSched};


pub mod machina;
pub mod args;
#[macro_use]
use::macros;

pub struct Peer {
    lines: Lines,
    tx_req: mpsc::UnboundedSender<Box<WorkerRequestContent>>,
    tx_sched: Arc<Mutex<TxMpscMainToSched>>,
    rx_toolbox: mpsc::UnboundedReceiver<Box<WorkerToPeerRequestAndPriority>>,
}

impl Peer {
    pub fn new(socket: TcpStream, tx_req: mpsc::UnboundedSender<Box<WorkerRequestContent>>,
               tx_sched: Arc<Mutex<TxMpscMainToSched>>,
               rx_toolbox: mpsc::UnboundedReceiver<Box<WorkerToPeerRequestAndPriority>>,
               ) -> Peer {
        // let addr = lines.socket.peer_addr().unwrap();

        Peer {
            lines: Lines::new(socket),
            tx_req: tx_req,
            tx_sched: tx_sched,
            rx_toolbox: rx_toolbox,
        }
    }
}

impl Future for Peer {
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<(), io::Error> {
        i!("poll called");

        let _ = self.lines.poll_flush()?;

        while let Async::Ready(line) = self.lines.poll()? {
            i!("Received line : {:?}", line);
            //e!("admin got polled!!");
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
