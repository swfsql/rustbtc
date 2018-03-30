//use errors::*;
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;

//use std::net::SocketAddr;
//use std::thread;

use tokio::io;
use tokio;
//use futures;
use futures::sync::{mpsc, oneshot};
//use futures::future::{select_all, Either};

//use std::collections::HashMap;
//use std::iter::FromIterator;

//use std::io::{Error, ErrorKind};
//use std::collections::BinaryHeap;
//use std::cmp::Ordering;

use std::ops::Deref;
use std::ops::DerefMut;

use scheduler::commons;
use admin;

use self::commons::{Rx_mpsc, WorkerRequestContent, WorkerRequest, WorkerResponse, WorkerRequestPriority, WorkerResponseContent};

/*use self::commons::{AddrReqId, RequestId, RequestPriority, Rx_mpsc, Rx_mpsc_sf, Rx_one, Tx_mpsc,
                    Tx_one, WorkerRequest, WorkerRequestContent, WorkerRequestPriority,
                    WorkerResponse, WorkerResponseContent};*/

use tokio_timer::*;
//use futures::*;
use std::time::*;

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

        println!("worker:: poll");

        let Inbox(ref mut rec, ref mut reqs) = self.inbox;
        loop {
            println!("worker:: loop 0");
            match rec.poll() {
                Ok(Async::Ready(Some(wrk_req))) => {
                    reqs.push(wrk_req);
                    println!("worker:: loop 0 ran");
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
                    println!("worker:: Hi! Request received: {:#?}", wrk_req);
                    WorkerResponse::Empty
                },
                WorkerRequest::Wait{delay} => {

                    println!("worker:: Hi! Request received: {:#?}", wrk_req);
                    let timer = Timer::default();
                    let sleep = timer.sleep(Duration::from_secs(delay));
                    sleep.wait();
                    WorkerResponse::Empty
                },
                WorkerRequest::PeerAdd{addr, wait_handhsake, tx_sched} => {

                    //println!("worker:: Hi! Request received: {:#?}", &wrk_req);
                    match TcpStream::connect(&addr).wait() {
                        Ok(socket) => {
                            let (tx, rx) = mpsc::unbounded();
                            {
                                let tx_sched_unlocked = tx_sched.lock().unwrap();
                                tx_sched_unlocked.unbounded_send(commons::Rx_peers(socket.peer_addr().unwrap(), rx.into_future()));
                            }
                            let peer = admin::Peer::new(socket, tx, tx_sched);
                            let peer_machina = admin::machina::Machina::start(peer).map(|_| ()).map_err(|_| ());
                            tokio::spawn(peer_machina);
                            WorkerResponse::PeerAdd(Some(addr))
                        },
                        Err(_) => {WorkerResponse::PeerAdd(None)},
                    }

                },
                _ => {
                    println!("worker:: Request received: {:#?}", wrk_req);
                    WorkerResponse::Empty
                },
            };

            println!("worker:: response sending.");
            tx_one.send(Ok(Box::new(WorkerResponseContent(resp, addr.clone()))));
            println!("worker:: response sent.");
            task::current().notify();
        }
        println!("worker:: returning not ready (end).");
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
