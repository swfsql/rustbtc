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

use scheduler::commons;

use self::commons::{
    Tx_mpsc,
    Rx_mpsc,
    Rx_mpsc_sf,
    Tx_one,
    Rx_one,
    WorkerRequest,
    RequestPriority,
    WorkerRequestPriority,
    WorkerRequestContent,
    WorkerResponse,
    WorkerResponseContent,
    RequestId,
    AddrReqId};

struct Worker {

}

impl Worker {
    fn new() -> Worker {
        Worker{}
    }
}

