use errors::*;
//use tokio::net::{TcpListener, TcpStream};
//use tokio::prelude::*;

use std::net::SocketAddr;
//use std::thread;

//use tokio::io;
use futures;
use futures::sync::{mpsc, oneshot};
//use futures::future::{select_all, Either};

//use std::collections::HashMap;
//use std::iter::FromIterator;

//use std::io::{Error, ErrorKind};
//use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct AddrReqId(pub SocketAddr, pub RequestId);

// peer <-> scheduler <-> worker
pub type Tx_mpsc = mpsc::Sender<Box<WorkerRequestContent>>;
pub type Rx_mpsc = mpsc::Receiver<Box<WorkerRequestContent>>;
pub type Rx_mpsc_sf = futures::stream::StreamFuture<Rx_mpsc>;
pub type Tx_one = oneshot::Sender<Result<Box<WorkerResponseContent>>>;
pub type Rx_one = oneshot::Receiver<Result<Box<WorkerResponseContent>>>;

#[derive(Debug)]
pub struct WorkerRequestContent(pub WorkerRequestPriority, pub Tx_one, pub AddrReqId);

impl Eq for WorkerRequestContent {}

impl PartialOrd for WorkerRequestContent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for WorkerRequestContent {
    fn cmp(&self, other: &WorkerRequestContent) -> Ordering {
        (self.0).1.cmp(&(other.0).1)
    }
}

impl PartialEq for WorkerRequestContent {
    fn eq(&self, other: &WorkerRequestContent) -> bool {
        (self.0).1 == (other.0).1
    }
}

#[derive(Debug)]
pub struct WorkerResponseContent(pub WorkerResponse, pub AddrReqId);

#[derive(Debug)]
pub enum WorkerRequest {
    NewPeer { addr: SocketAddr },
    KillPeer { addr: SocketAddr },
    InfoPeer { addr: SocketAddr },
    ListPeers,
    SendPing { addr: SocketAddr },
    Hello,
}

pub type RequestPriority = u8;
pub type RequestId = usize;

#[derive(Debug)]
pub struct WorkerRequestPriority(WorkerRequest, RequestPriority);

#[derive(Debug)]
pub enum WorkerResponse {
    String(String),
    Bool(bool),
}





/*


if let Bool(bool_interno) = reposta {

} else {
    panic!("allsad");
}

match resposta {
    Bool(pega_a_bool) => {},
    _ => panic!(),
}

*/