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

type RequestPriority = u8;
#[derive(Debug)]
struct WorkerRequestPriority(WorkerRequest, RequestPriority);

#[derive(Debug)]
enum WorkerResponse {
    String(String),
    Bool(bool),
}

type RequestId = usize;

#[derive(Hash,Eq,PartialEq,Debug)]
struct AddrReqId (SocketAddr, RequestId);

// peer <-> scheduler <-> worker
type Tx_mpsc = mpsc::Sender<(WorkerRequestPriority,
    Tx_one,
    AddrReqId)>;
type Rx_mpsc = mpsc::Receiver<(WorkerRequestPriority,
    Tx_one,
    AddrReqId)>;
type Rx_mpsc_sf = futures::stream::StreamFuture<Rx_mpsc>;
type Tx_one = oneshot::Sender<(WorkerResponse,
    AddrReqId)>;
type Rx_one = oneshot::Receiver<(WorkerResponse,
    AddrReqId)>;

struct Worker {

}

impl Worker {
    fn new() -> Worker {
        Worker{}
    }
}

struct Outbox(Tx_mpsc, HashMap<AddrReqId, Rx_one>);

impl Eq for Outbox { }

impl PartialOrd for Outbox {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Outbox {
    fn cmp(&self, other: &Outbox) -> Ordering {
        self.1.len().cmp(&other.1.len())
    }
}

impl PartialEq for Outbox {
    fn eq(&self, other: &Outbox) -> bool {
        self.1.len() == other.1.len()
    }
}

struct Scheduler {
        // tuple(Rx from peer mpsc; Tx to peer oneshot (after received on each request))
    inbox: HashMap<SocketAddr, (Rx_mpsc_sf, HashMap<RequestId, Tx_one>)>,
        // selector: Vec<_>,
        // tuple(Tx to worker mpsc; Rx from worker oneshot)
    outbox: Vec<Outbox>
}


impl Future for Scheduler {
    type Item = ();
    type Error = Error;

    fn poll(&mut self) -> Poll<(), Error> {

        loop {

          let ret = {
            //
            let outbox_rx_one = futures::future::select_all(
                self.outbox
                    .iter_mut()
                    .flat_map(|&mut Outbox(_, ref mut rx_one_hm)|
                        rx_one_hm
                            .iter_mut()
                            .map(|(_, fut)| fut)));
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
                  let (wrkmsg, rx_one, addr_req_id) = first.unwrap();

                  // the tail must be taken out of this scope, because
                  // there's no replace access into the self.inbox
                  Ok(Async::Ready(Some((rx_one, addr_req_id, tail_stream))))
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
            Ok(Async::Ready(Some((rx_one, addr_req_id, tail_stream)))) => {

              let &mut(ref mut prev_rx_mpsc_sf, ref mut prev_oneshots) =
                self.inbox.get_mut(&addr_req_id.0).unwrap();
              *prev_rx_mpsc_sf = tail_stream.into_future();
              if prev_oneshots.contains_key(&addr_req_id.1) {
                  println!("Error: colliding oneshot key");
                  panic!("TODO");
              }
              prev_oneshots.insert(addr_req_id.1, rx_one);

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
                scheduler::WorkerRequestPriority,
                futures::Sender<(scheduler::WorkerResponse, std::net::SocketAddr)>,
                std::net::SocketAddr
            )>,

            // restante da stream do único canal escolhido
            futures::sync::mpsc::Receiver<(
                scheduler::WorkerRequestPriority,
                futures::Sender<(scheduler::WorkerResponse, std::net::SocketAddr)>,
                std::net::SocketAddr
            )>
        ),

        // posição [antiga] do elemento no vetor
        usize,

        // restante do vetor de todos os canais (que estão no hashmap)
        std::vec::Vec<&mut futures::stream::StreamFuture<futures::sync::mpsc::Receiver<(
            scheduler::WorkerRequestPriority,
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

