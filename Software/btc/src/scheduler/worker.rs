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

use std::ops::Deref;
use std::ops::DerefMut;

use scheduler::commons;

use self::commons::{Rx_mpsc, WorkerRequestContent, WorkerRequest, WorkerResponse, WorkerRequestPriority, WorkerResponseContent};

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
        if let Some(mut req) = reqs.pop() {
            let mut req = *req;
            let WorkerRequestContent(
                WorkerRequestPriority(wrk_req, _req_pri),
                tx_one,
                addr) = req;
            let resp = match wrk_req {
                WorkerRequest::Hello => {
                    println!("Hi! Request received: {:#?}", wrk_req);
                    WorkerResponse::Empty
                },
                _ => {
                    println!("Request received: {:#?}", wrk_req);
                    WorkerResponse::Empty
                },
            };

            tx_one.send(Ok(Box::new(WorkerResponseContent(resp, addr.clone()))));
            task::current().notify();
        }
        Ok(Async::NotReady)
    }
}


/*
pub struct WorkerRequestContent(
    pub WorkerRequestPriority,
    pub Tx_one,
    pub AddrReqId);
pub struct WorkerRequestPriority(
    WorkerRequest,
    RequestPriority);
*/
/*

pub struct WorkerResponseContent(pub WorkerResponse, pub AddrReqId);

#[derive(Debug)]
pub enum WorkerResponse {
    Empty,
    String(String),
    Bool(bool),
}

*/
