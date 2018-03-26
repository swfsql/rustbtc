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

use std::io::{Error, ErrorKind};

#[derive(Debug)]
enum WorkerMsg {
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
    }
}

#[derive(Debug)]
enum WorkerResult {
    String(String),
    Bool(bool),
}

type Tx_mpsc = mpsc::Sender<(WorkerMsg,
    oneshot::Sender<Option<WorkerResult>>,
    SocketAddr)>;
type Rx_mpsc = mpsc::Receiver<(WorkerMsg,
    oneshot::Receiver<Option<WorkerResult>>,
    SocketAddr)>;
type Rx_mpsc_sf = futures::stream::StreamFuture<Rx_mpsc>;
type Tx_one = oneshot::Sender<(WorkerResult,
    SocketAddr)>;
type Rx_one = oneshot::Receiver<(WorkerResult,
    SocketAddr)>;

struct Scheduler {
    // tuple(Rx from peer mpsc; Tx to peer oneshot)
    inbox: HashMap<SocketAddr, (Rx_mpsc_sf, Tx_one)>,
    // selector: Vec<_>,
    // tuple(Tx to worker mpsc; Rx from worker oneshot)
    outbox: Vec<(Tx_mpsc, Rx_one)>,
}

impl Future for Scheduler {
    type Item = ();
    type Error = Error;

    fn poll(&mut self) -> Poll<(), Error> {

        ////Ready( (primeiro future, resto da stream) )

        let outbox_rx_one = futures::future::select_all(self.outbox.iter_mut()
            .map(|&mut(_, ref mut rx_one)| rx_one));

        let inbox_rx_mpsc = futures::future::select_all(self.inbox.iter_mut()
            .map(|(_, &mut(ref mut rx_mpsc, _))| rx_mpsc));
        // futures::selec_all(outbox.iter().map(|_, v| v))
        let mut fut = outbox_rx_one.select2(inbox_rx_mpsc)
          .map_err(|_| Error::new(ErrorKind::Other, "TODO: error in select2 for sched!")); // TODO: change

        loop {
            let res = try_ready!(fut.poll());
            match res {
                Either::A(rx_one) => {
                    println!("parabéns, recebido rx_one: {:#?}", rx_one);
                    return Ok(Async::Ready(()));
                },
                Either::B(rx_mpsc) => {
                    println!("parabéns, recebido rx_mpsc: {:#?}", rx_mpsc);
                    return Ok(Async::Ready(()));
                },
                //Err(_) => {
                //    println!("Error at scheduler");
                //},
                //
                //Err(Either::A((e, _))) => err(e).boxed(),
                //Err(Either::B((e, _))) => err(e).boxed(),
            }
        }

        Ok(Async::NotReady)
    }
}
