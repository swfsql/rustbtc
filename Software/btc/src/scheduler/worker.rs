//use errors::*;
//use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;

//use std::net::SocketAddr;
//use std::thread;

use tokio::io;
//use futures;
//use futures::sync::{mpsc, oneshot};
//use futures::future::{select_all, Either};

//use std::collections::HashMap;
//use std::iter::FromIterator;

//use std::io::{Error, ErrorKind};
//use std::collections::BinaryHeap;
//use std::cmp::Ordering;

use scheduler::commons;

use self::commons::{Rx_mpsc, WorkerRequestContent};

/*use self::commons::{AddrReqId, RequestId, RequestPriority, Rx_mpsc, Rx_mpsc_sf, Rx_one, Tx_mpsc,
                    Tx_one, WorkerRequest, WorkerRequestContent, WorkerRequestPriority,
                    WorkerResponse, WorkerResponseContent};*/

struct Inbox(Rx_mpsc, Vec<Box<WorkerRequestContent>>);

pub struct Worker {
    inbox: Inbox,
}

/*
pub type Rx_mpsc = mpsc::Receiver<WorkerRequestContent>;

pub struct WorkerRequestContent(
  pub WorkerRequestPriority,
  pub Tx_one,
  pub AddrReqId);
*/

impl Worker {
    pub fn new(rx_mpsc: Rx_mpsc) -> Worker {
        Worker {
            inbox: Inbox(rx_mpsc, vec![]),
        }
    }
}

impl Future for Worker {
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<(), io::Error> {
        let Inbox(ref mut rec, ref mut reqs) = self.inbox;
        loop {
            match rec.poll() {
                Ok(Async::Ready(Some(wrk_req))) => {
                    reqs.push(wrk_req);
                }
                Ok(Async::NotReady) => break,
                _ => panic!("Unexpected value for worker polling on reader channel"),
            };
        }

        reqs.sort_unstable();
        if let Some(req) = reqs.iter().rev().next() {
            let resp = match req {
                _ => {
                    println!("Request received: {:#?}", req);
                    ()
                }
            };
            task::current().notify();
        }
        Ok(Async::NotReady)
    }
}
