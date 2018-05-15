pub mod channel_content;
use std::net::SocketAddr;
//use std::thread;

//use tokio::io;
use futures;
use futures::sync::{mpsc, oneshot};
//use futures::future::{select_all, Either};

//use std::iter::FromIterator;

//use std::io::{Error, ErrorKind};
//use std::collections::BinaryHeap;

use errors::*;
//use futures::future::{select_all, Either};
use self::channel_content::{
    MainToSchedRequestContent, RouterToPeerRequestAndPriority, SchedToRouterRequestContent,
    SchedulerResponse, WorkerRequestContent, WorkerResponseContent,
    WorkerToBlockChainRequestContent, WorkerToBlockChainResponse, WorkerToRouterRequestContent,
    WorkerToRouterResponse,
};

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

// worker -> blockchain
pub type TxOneWorkerToBlockChain = Option<oneshot::Sender<Box<WorkerToBlockChainResponse>>>;

// TODO maybe remove
// peer <-> scheduler <-> worker
pub struct TxPeers(pub SocketAddr, pub RxMpscSf);

// scheduler -> main/worker
pub type TxRegOne = oneshot::Sender<Box<SchedulerResponse>>;
// main/worker <- scheduler
pub type RxRegOne = oneshot::Receiver<Box<SchedulerResponse>>;
