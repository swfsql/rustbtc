use std::net::SocketAddr;
//use std::thread;

//use tokio::io;
//use futures;
//use futures::sync::{mpsc, oneshot};
//use futures::future::{select_all, Either};

use std::collections::HashMap;
//use std::iter::FromIterator;

//use std::io::{Error, ErrorKind};
//use std::collections::BinaryHeap;
use std::cmp::Ordering;

use actor::commons::{
    RxMpscSf, TxMpscMainToSched, TxMpscRouterToPeer, TxOne, TxOneWorkerToBlockChain,
    TxOneWorkerToRouter, TxRegOne,
};
//use codec;
use codec::msgs::msg::Msg;
use errors::*;
use std;
use std::fmt;
pub type RequestPriority = u8;
pub type RequestId = usize;
pub type ActorId = usize;
pub type RawMsg = Vec<u8>;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct AddrReqId(pub ActorId, pub RequestId);

/////////////////////////////////////////////
///////////////////WORKER////////////////////
/////////////////////////////////////////////
#[derive(Clone)]
pub enum WorkerRequest {
    PeerAdd {
        addr: SocketAddr,
        wait_handshake: bool,
        tx_sched: TxMpscMainToSched,
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
    NewVerack,
    NewGetHeaders,
}

impl std::fmt::Debug for WorkerRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            WorkerRequest::PeerAdd {
                addr: _,
                wait_handshake: _,
                tx_sched: _,
            } => format!("PeerAdd"),
            WorkerRequest::PeerRemove { actor_id: _ } => format!("PeerRemove"),
            WorkerRequest::PeerGetInfo { actor_id: _ } => format!("PeerGetInfo"),
            WorkerRequest::Wait { delay: _ } => format!("Wait"),
            WorkerRequest::ListPeers => format!("ListPeers"),
            WorkerRequest::PeerPrint => format!("PeerPrint"),
            WorkerRequest::MsgFromHex { send: _, binary: _ } => format!("MsgFromHex"),
            _ => format!("some other"), //
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
    MsgFromHex(Result<Msg>),
    ListPeers(HashMap<ActorId, SocketAddr>),
    Version(Msg),
    Verack(Msg),
    GetHeaders(Msg)
}

#[derive(Clone, Debug)]
pub struct WorkerRequestPriority(pub WorkerRequest, pub RequestPriority);

#[derive(Debug)]
pub struct WorkerRequestContent(pub WorkerRequestPriority, pub TxOne, pub AddrReqId);

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
/////////////////////////////////////////////
////////////////////PEER/////////////////////
/////////////////////////////////////////////
#[derive(Debug, Clone)]
pub enum PeerRequest {
    Dummy,
    SelfRemove,
    Forward(RawMsg),
    HandShake(RawMsg),
}
/////////////////////////////////////////////
//////////////////SCHEDULER//////////////////
/////////////////////////////////////////////
pub enum SchedulerResponse {
    RegisterResponse(Result<usize>),
    UnregisterResponse(Result<()>),
}
/////////////////////////////////////////////
//////////////WORKER TO ROUTER///////////////
/////////////////////////////////////////////
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
    MsgFromHex(Result<Msg>),
    ListPeers(HashMap<ActorId, SocketAddr>),
}

pub struct WorkerToRouterRequestContent(pub WorkerToRouterRequest, pub TxOneWorkerToRouter);
/////////////////////////////////////////////
////////////WORKER TO BLOCKCHAIN/////////////
/////////////////////////////////////////////
#[derive(Clone)]
pub enum WorkerToBlockChainRequest {
    ListPeers,
    MsgToPeer(ActorId, Box<RouterToPeerRequestAndPriority>),
    MsgToAllPeers(Box<RouterToPeerRequestAndPriority>),
    PeerRemove(ActorId, Box<RouterToPeerRequestAndPriority>),
}
pub struct WorkerToBlockChainRequestContent(
    pub WorkerToBlockChainRequest,
    pub TxOneWorkerToBlockChain,
);

#[derive(Debug)]
pub enum WorkerToBlockChainResponse {
    Empty,
    String(String),
    Bool(bool),
    PeerAdd(Option<SocketAddr>),
    PeerRemove(bool),
    MsgFromHex(Result<Msg>),
    ListPeers(HashMap<ActorId, SocketAddr>),
}
/////////////////////////////////////////////
///////////////ROUTER TO PEER////////////////
/////////////////////////////////////////////
#[derive(Clone)]
pub struct RouterToPeerRequestAndPriority(pub PeerRequest, pub RequestPriority);

/////////////////////////////////////////////
//////////////SCHEDULER TO ROUTER////////////
/////////////////////////////////////////////
pub enum SchedToRouterRequestContent {
    Register(ActorId, SocketAddr, TxMpscRouterToPeer),
    Unregister(ActorId),
}
/////////////////////////////////////////////
///////////////MAIN TO SCHEDULER/////////////
/////////////////////////////////////////////
pub enum MainToSchedRequestContent {
    Register(SocketAddr, RxMpscSf, TxMpscRouterToPeer, TxRegOne),
    Unregister(ActorId),
}
