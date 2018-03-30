#[derive(Debug)]
pub enum WorkerRequest {
    PeerAdd { addr: SocketAddr, wait_handhsake: bool ,tx_sched: Arc<Mutex<mpsc::UnboundedSender<Rx_peers>>>},
    KillPeer { addr: SocketAddr },
    InfoPeer { addr: SocketAddr },
    ListPeers,
    SendPing { addr: SocketAddr },
    Hello,
    Wait { delay: u64 },
}

#[derive(Debug)]
pub enum WorkerResponse {
    Empty,
    String(String),
    Bool(bool),
    PeerAdd(Option<SocketAddr>),
}

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
use std::sync::{Arc, Mutex};

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct AddrReqId(pub SocketAddr, pub RequestId);

// peer <-> scheduler <-> worker
pub type Tx_mpsc = mpsc::UnboundedSender<Box<WorkerRequestContent>>;
pub type Rx_mpsc = mpsc::UnboundedReceiver<Box<WorkerRequestContent>>;
pub type Rx_mpsc_sf = futures::stream::StreamFuture<Rx_mpsc>;
pub type Tx_one = oneshot::Sender<Result<Box<WorkerResponseContent>>>;
pub type Rx_one = oneshot::Receiver<Result<Box<WorkerResponseContent>>>;

#[derive(Debug)]
pub struct Rx_peers (pub SocketAddr, pub Rx_mpsc_sf);
#[derive(Debug)]
pub struct Tx_peers (pub SocketAddr, pub Rx_mpsc_sf);

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



pub type RequestPriority = u8;
pub type RequestId = usize;

#[derive(Debug)]
pub struct WorkerRequestPriority(pub WorkerRequest, pub RequestPriority);






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