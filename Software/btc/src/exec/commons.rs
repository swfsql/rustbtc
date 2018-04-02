#[derive(Debug)]
pub enum WorkerRequest {
    PeerAdd { addr: SocketAddr, wait_handhsake: bool ,tx_sched: Arc<Mutex<TxMpscMainToSched>>},
    PeerKill{ addr: SocketAddr },
    PeerGetInfo { addr: SocketAddr },
    ListPeers,
    SendPing { addr: SocketAddr },
    Hello,
    Wait { delay: u64 },
    PeerPrint,
}

#[derive(Debug)]
pub enum WorkerResponse {
    Empty,
    String(String),
    Bool(bool),
    PeerAdd(Option<SocketAddr>),
}

#[derive(Debug)]
pub enum PeerRequest {
    Dummy,
}

pub struct ToolBox {
    pub peer_messenger: Mutex<HashMap<SocketAddr, TxMpscWorkerToPeer>>,
}

impl ToolBox {
    pub fn new() -> ToolBox {
        ToolBox {
            peer_messenger: Mutex::new(HashMap::new()),
        }
    }
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

use std::collections::HashMap;
//use std::iter::FromIterator;

//use std::io::{Error, ErrorKind};
//use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::sync::{Arc, Mutex};

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct AddrReqId(pub SocketAddr, pub RequestId);

// peer <-> scheduler <-> worker

// peer/admin -> scheduler -> worker
pub type TxMpsc = mpsc::UnboundedSender<Box<WorkerRequestContent>>;
// worker <- scheduler <- peer/admin
pub type RxMpsc = mpsc::UnboundedReceiver<Box<WorkerRequestContent>>;
//
pub type RxMpscSf = futures::stream::StreamFuture<RxMpsc>;
// worker -> scheduler -> peer
pub type TxOne = oneshot::Sender<Result<Box<WorkerResponseContent>>>;
// peer <- scheduler <- worker
pub type RxOne = oneshot::Receiver<Result<Box<WorkerResponseContent>>>;
// worker [with toolbox] -> peer
pub type TxMpscWorkerToPeer = mpsc::UnboundedSender<Box<WorkerToPeerRequestAndPriority>>;
// peer <- worker [with toolbox]
pub type RxMpscWorkerToPeer = mpsc::UnboundedReceiver<Box<WorkerToPeerRequestAndPriority>>;
// main/.. -> scheduler
pub type TxMpscMainToSched = mpsc::UnboundedSender<Box<MainToSchedRequestContent>>;
// scheduler <- main/..
pub type RxMpscMainToSched = mpsc::UnboundedReceiver<Box<MainToSchedRequestContent>>;



#[derive(Debug)]
pub struct RxPeers (pub SocketAddr, pub RxMpscSf);
#[derive(Debug)]
pub struct TxPeers (pub SocketAddr, pub RxMpscSf);

#[derive(Debug)]
pub struct WorkerRequestContent(pub WorkerRequestPriority, pub TxOne, pub AddrReqId);

#[derive(Debug)]
pub struct MainToSchedRequestContent(pub RxPeers, pub TxMpscWorkerToPeer);

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
#[derive(Debug)]
pub struct WorkerToPeerRequestAndPriority(pub PeerRequest, pub RequestPriority);






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