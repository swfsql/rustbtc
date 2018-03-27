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

#[derive(Debug)]
enum WorkerRequest {
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
enum WorkerResponse {
    String(String),
    Bool(bool),
}

type RequestId = usize;

// peer <-> scheduler <-> worker
type Tx_mpsc = mpsc::Sender<(WorkerRequest,
    Tx_one,
    SocketAddr,
    RequestId)>;
type Rx_mpsc = mpsc::Receiver<(WorkerRequest,
    Tx_one,
    SocketAddr,
    RequestId)>;
type Rx_mpsc_sf = futures::stream::StreamFuture<Rx_mpsc>;
type Tx_one = oneshot::Sender<(WorkerResponse,
    SocketAddr,
    RequestId)>;
type Rx_one = oneshot::Receiver<(WorkerResponse,
    SocketAddr,
    RequestId)>;


struct Scheduler {
        // tuple(Rx from peer mpsc; Tx to peer oneshot (after received on each request))
    inbox: HashMap<SocketAddr, (Rx_mpsc_sf, HashMap<RequestId, Tx_one>)>,
        // selector: Vec<_>,
        // tuple(Tx to worker mpsc; Rx from worker oneshot)
    outbox: Vec<(Tx_mpsc, Rx_one)>,
}

impl Future for Scheduler {
    type Item = ();
    type Error = Error;

    fn poll(&mut self) -> Poll<(), Error> {

        loop {

          let ret = {
            //
            let outbox_rx_one = futures::future::select_all(self.outbox.iter_mut()
                .map(|&mut(_, ref mut rx_one)| rx_one));
            // select all from first futures from channel stream
            let inbox_rx_mpsc = futures::future::select_all(self.inbox.iter_mut()
                .map(|(_, &mut(ref mut rx_mpsc, _))| rx_mpsc));
            //
            let mut fut = outbox_rx_one.select2(inbox_rx_mpsc)
            .map_err(|_| Error::new(ErrorKind::Other, "TODO: error in select2 for sched!")); // TODO: change

            // Either::B needed to get one value out of this next scope,
            // so returning that vlaue was needed.
            // other returns are normal future workflow.
            match fut.poll() {
              //
              Ok(Async::Ready(Either::A(rx_one))) => {
                  println!("parabéns, recebido rx_one: {:#?}", rx_one);
                  Ok(Async::Ready(None)) // TODO
              },
              //
              Ok(Async::Ready(Either::B((((first, tail_stream),
                      _index, _vec_other_mpsc), _other_either)))) => {

                  println!("parabéns, recebido rx_mpsc: {:#?}", first);
                  let (wrkmsg, rx_one, addr, req_id) = first.unwrap();

                  // the tail must be taken out of this scope, because
                  // there's no replace access into the self.inbox
                  Ok(Async::Ready(Some((rx_one, addr, tail_stream, req_id))))
                  // there is already a &mut to self.inbox in a scope outer to the
                  // tail_stream. Solution: move tail_stream out, and then a new
                  // &mut access to self.inbox is available.

                  // also, it was necessary to replace one old value with this new
                  // one (the tail), because the old one had a future has was consumed,
                  // and inside that future there was the tail stream. So from the tail
                  // stream, a new first future will be placed back for that
                  // channel (inbox).

              },
              Ok(Async::NotReady) => Ok(Async::NotReady),
              Err(e) => Err(e),
            }

          };

        //
          match ret {
            // replace the tail's first future (that will contain the next tail)
            // back into the channel (inbox)
            Ok(Async::Ready(Some((rx_one, addr, tail_stream, req_id)))) => {

              let &mut(ref mut prev_rx_mpsc_sf, ref mut prev_oneshots) =
                self.inbox.get_mut(&addr).unwrap();
              *prev_rx_mpsc_sf = tail_stream.into_future();
              if prev_oneshots.contains_key(&req_id) {
                  println!("Error: colliding oneshot key");
                  panic!("TODO");
              }
              prev_oneshots.insert(req_id, rx_one);

              /*self.inbox
                .entry(addr)
                .and_modify(move |inbox_entry| {
                  inbox_entry.0 = tail_stream.into_future();
                });*/
            },
            _ => {}
          };


        }

        Ok(Async::NotReady)
    }
}

/*


(
    (
        (
            // primeiro elemento da stream
            std::option::Option<(
                scheduler::WorkerRequest,
                futures::Sender<(scheduler::WorkerResponse, std::net::SocketAddr)>,
                std::net::SocketAddr
            )>,

            // restante da stream do único canal escolhido
            futures::sync::mpsc::Receiver<(
                scheduler::WorkerRequest,
                futures::Sender<(scheduler::WorkerResponse, std::net::SocketAddr)>,
                std::net::SocketAddr
            )>
        ),

        // posição [antiga] do elemento no vetor
        usize,

        // restante do vetor de todos os canais (que estão no hashmap)
        std::vec::Vec<&mut futures::stream::StreamFuture<futures::sync::mpsc::Receiver<(
            scheduler::WorkerRequest,
            futures::Sender<(scheduler::WorkerResponse, std::net::SocketAddr)>,
            std::net::SocketAddr
        )>>>
    ),

    // future do Either::A
    futures::SelectAll<&mut futures::Receiver<(scheduler::WorkerResponse, std::net::SocketAddr)>>

)`


*/




/*



*/

