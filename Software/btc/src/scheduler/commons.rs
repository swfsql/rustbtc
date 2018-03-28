use errors::*;
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;

use std::net::SocketAddr;
use std::thread;


use tokio::io;
use futures;
use futures::sync::{mpsc,oneshot};
use futures::future::{select_all, Either};

use std::collections::HashMap;
use std::iter::FromIterator;

use std::io::{Error, ErrorKind};
use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Hash,Eq,PartialEq,Debug)]
pub struct AddrReqId (pub SocketAddr, pub RequestId);

// peer <-> scheduler <-> worker
pub type Tx_mpsc = mpsc::Sender<WorkerRequestContent>;
pub type Rx_mpsc = mpsc::Receiver<WorkerRequestContent>;
pub type Rx_mpsc_sf = futures::stream::StreamFuture<Rx_mpsc>;
pub type Tx_one = oneshot::Sender<(WorkerResponse,
    AddrReqId)>;
pub type Rx_one = oneshot::Receiver<(WorkerResponse,
    AddrReqId)>;


#[derive(Debug)]
pub struct WorkerRequestContent(
  pub WorkerRequestPriority,
  pub Tx_one,
  pub AddrReqId);

#[derive(Debug)]
pub struct WorkerResponseContent(
  pub WorkerResponse,
  pub AddrReqId);

#[derive(Debug)]
pub enum WorkerRequest {
    NewPeer {
        addr: SocketAddr,
    },
    KillPeer {
        addr: SocketAddr,
    },
    InfoPeer {
        addr: SocketAddr,
    },
    ListPeers,
    SendPing {
        addr: SocketAddr,
    },
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



