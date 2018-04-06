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

use exec::commons::{WorkerRequestContent,RxPeers,WorkerToPeerRequestAndPriority,ToolBox,TxMpscMainToSched,RxOne};


pub mod machina;
#[macro_use]
use::macros;

pub struct Peer {
    lines: Lines,
    rx_ignored: Vec<RxOne>,
    tx_req: mpsc::UnboundedSender<Box<WorkerRequestContent>>,
    tx_sched: Arc<Mutex<TxMpscMainToSched>>,
    rx_toolbox: mpsc::UnboundedReceiver<Box<WorkerToPeerRequestAndPriority>>,
    request_counter: usize,
}

impl Peer {
    pub fn new(socket: TcpStream, tx_req: mpsc::UnboundedSender<Box<WorkerRequestContent>>,
               tx_sched: Arc<Mutex<TxMpscMainToSched>>,
               rx_toolbox: mpsc::UnboundedReceiver<Box<WorkerToPeerRequestAndPriority>>,
               ) -> Peer {
        // let addr = lines.socket.peer_addr().unwrap();

        Peer {
            lines: Lines::new(socket),
            rx_ignored: Vec::new(),
            tx_req: tx_req,
            tx_sched: tx_sched,
            rx_toolbox: rx_toolbox,
            request_counter: 0,
        }
    }

    pub fn poll_ignored(&mut self) {
            let removed_indices = self.rx_ignored
                .iter_mut()
                .enumerate()
                .map(|(i, rx)| (i, rx.poll()))
                .filter(|&(i, ref fut)|
                    fut.is_err() || (fut.is_ok() && fut.as_ref().unwrap().is_ready()))
                .inspect(|&(_i, ref rx)| i!("Oneshot response arrived, and got ignored: \n{:#?}", rx))
                .map(|(i, _rx)| i)
                .collect::<Vec<_>>();
            for i in removed_indices.iter().rev() {
                self.rx_ignored.swap_remove(*i);
            }

    }

    pub fn next_request_counter(&mut self) -> usize {
        self.request_counter += 1;
        self.request_counter
    }

    pub fn push_ignored(&mut self, rx: RxOne) {
        self.rx_ignored.push(rx);
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
