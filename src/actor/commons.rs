#[derive(Clone)]
pub enum WorkerRequest {
    PeerAdd {
        addr: SocketAddr,
        wait_handshake: bool,
        tx_sched: Arc<Mutex<TxMpscMainToSched>>,
    },
    PeerRemove {
        actor_id: ActorId,
    },
    PeerGetInfo {
        actor_id: ActorId,
    },
    ListPeers,
    SendPing {
        actor_id: ActorId,
    },
    Hello,
    Wait {
        delay: u64,
    },
    PeerPrint,
    MsgFromHex {
        send: bool,
        binary: Vec<u8>,
    },
    NewVersion {
        addr: SocketAddr,
    },
    NewVerack{
        version: Msg,
    }
}

use codec::msgs::msg::Msg;

use std;
use std::fmt;
impl std::fmt::Debug for WorkerRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            WorkerRequest::PeerAdd{addr: _, wait_handshake: _, tx_sched: _} => format!("PeerAdd"),
            WorkerRequest::PeerRemove{actor_id: _} => format!("PeerRemove"),
            WorkerRequest::PeerGetInfo{actor_id: _} => format!("PeerGetInfo"),
            WorkerRequest::Wait{delay: _} => format!("Wait"),
            WorkerRequest::ListPeers => format!("ListPeers"),
            WorkerRequest::PeerPrint => format!("PeerPrint"),
            WorkerRequest::MsgFromHex{send: _, binary: _} => format!("MsgFromHex"),
            
            _ => format!("some other"),//
            
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub enum WorkerResponse {
    Empty,
    String(String),
    Bool(bool),
    PeerAdd(Option<SocketAddr>),
    PeerRemove(bool),
    MsgFromHex(Result<codec::msgs::msg::Msg>),
    ListPeers(HashMap<ActorId, SocketAddr>),
    Version(codec::msgs::msg::Msg),
    Verack(codec::msgs::msg::Msg),
}

#[derive(Clone)]
pub enum WorkerToRouterRequest {
    ListPeers,
    MsgToPeer(ActorId, Box<RouterToPeerRequestAndPriority>),
    MsgToAllPeers(Box<RouterToPeerRequestAndPriority>),
    PeerRemove(ActorId, Box<RouterToPeerRequestAndPriority>),
}


#[derive(Debug)]
pub enum WorkerToRouterResponse {
    Empty,
    String(String),
    Bool(bool),
    PeerAdd(Option<SocketAddr>),
    PeerRemove(bool),
    MsgFromHex(Result<codec::msgs::msg::Msg>),
    ListPeers(HashMap<ActorId, SocketAddr>),
}

#[derive(Clone)]
pub enum WorkerToBlockChainRequest {
    ListPeers,
    MsgToPeer(ActorId, Box<RouterToPeerRequestAndPriority>),
    MsgToAllPeers(Box<RouterToPeerRequestAndPriority>),
    PeerRemove(ActorId, Box<RouterToPeerRequestAndPriority>),
}

#[derive(Debug)]
pub enum WorkerToBlockChainResponse {
    Empty,
    String(String),
    Bool(bool),
    PeerAdd(Option<SocketAddr>),
    PeerRemove(bool),
    MsgFromHex(Result<codec::msgs::msg::Msg>),
    ListPeers(HashMap<ActorId, SocketAddr>),
}

#[derive(Debug, Clone)]
pub enum PeerRequest {
    Dummy,
    SelfRemove,
    Forward(RawMsg),
    HandShake(RawMsg),
}

pub struct ToolBox {
    pub peer_messenger: Mutex<HashMap<SocketAddr, TxMpscRouterToPeer>>,
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

use codec;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct AddrReqId(pub ActorId, pub RequestId);

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

// router -> peer
pub type TxMpscRouterToPeer = mpsc::UnboundedSender<Box<RouterToPeerRequestAndPriority>>;
// peer <- router
pub type RxMpscRouterToPeer = mpsc::UnboundedReceiver<Box<RouterToPeerRequestAndPriority>>;

// main/.. -> scheduler
pub type TxMpscMainToSched = mpsc::UnboundedSender<Box<MainToSchedRequestContent>>;
// scheduler <- main/..
pub type RxMpscMainToSched = mpsc::UnboundedReceiver<Box<MainToSchedRequestContent>>;

// router -> worker
pub type TxOneWorkerToRouter = Option<oneshot::Sender<Box<WorkerToRouterResponse>>>;
// worker <- router
pub type RxOneWorkerToRouter = Option<oneshot::Receiver<Box<WorkerToRouterResponse>>>;

// worker -> router
pub type TxMpscWorkerToRouter = mpsc::UnboundedSender<Box<WorkerToRouterRequestContent>>;
// router <- worker
pub type RxMpscWorkerToRouter = mpsc::UnboundedReceiver<Box<WorkerToRouterRequestContent>>;

// sched -> router
pub type TxMpscSchedToRouter = mpsc::UnboundedSender<Box<SchedToRouterRequestContent>>;
// router <- sched
pub type RxMpscSchedToRouter = mpsc::UnboundedReceiver<Box<SchedToRouterRequestContent>>;

// blockchain -> worker
pub type RxMpscWorkerToBlockChain = mpsc::UnboundedReceiver<Box<WorkerToBlockChainRequestContent>>;
// worker -> blockchain
pub type TxMpscWorkerToBlockChain = mpsc::UnboundedSender<Box<WorkerToBlockChainRequestContent>>;

pub struct WorkerToBlockChainRequestContent(pub WorkerToBlockChainRequest, pub TxOneWorkerToBlockChain);

pub type TxOneWorkerToBlockChain = Option<oneshot::Sender<Box<WorkerToBlockChainResponse>>>;

// TODO maybe remove
pub struct TxPeers(pub SocketAddr, pub RxMpscSf);

pub enum SchedToRouterRequestContent {
    Register(ActorId, SocketAddr, TxMpscRouterToPeer),
    Unregister(ActorId),
}

pub struct WorkerToRouterRequestContent(pub WorkerToRouterRequest, pub TxOneWorkerToRouter);

#[derive(Debug)]
pub struct WorkerRequestContent(pub WorkerRequestPriority, pub TxOne, pub AddrReqId);

pub enum MainToSchedRequestContent {
    Register(SocketAddr, RxMpscSf, TxMpscRouterToPeer, TxRegOne),
    Unregister(ActorId),
}

pub type ActorId = usize;
pub type RawMsg = Vec<u8>;

// scheduler -> main/worker
pub type TxRegOne = oneshot::Sender<Box<SchedulerResponse>>;
// main/worker <- scheduler
pub type RxRegOne = oneshot::Receiver<Box<SchedulerResponse>>;


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
pub enum SchedulerResponse {
    RegisterResponse(Result<usize>),
    UnregisterResponse(Result<()>),
}

pub type RequestPriority = u8;
pub type RequestId = usize;

#[derive(Clone,Debug)]
pub struct WorkerRequestPriority(pub WorkerRequest, pub RequestPriority);

#[derive(Clone)]
pub struct RouterToPeerRequestAndPriority(pub PeerRequest, pub RequestPriority);
