use errors::*;
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;

use std::net::SocketAddr;
use std::thread;

use tokio;
use tokio::io;
use futures;
use futures::sync::{mpsc,oneshot};
use futures::future::{select_all, Either};

use std::collections::HashMap;
use std::iter::FromIterator;

use std::io::{Error, ErrorKind};
use std::collections::BinaryHeap;
use std::cmp::Ordering;

mod worker;
mod commons;

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


struct Inbox(Rx_mpsc_sf, HashMap<RequestId, Tx_one>);
struct Outbox(Tx_mpsc, HashMap<AddrReqId, Rx_one>);

impl Outbox {
  fn new(tx: Tx_mpsc) -> Outbox {
    Outbox(tx, HashMap::new())

  }
}

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
    inbox: HashMap<SocketAddr, Inbox>,
    outbox: Vec<Outbox>
}

enum AorB<C, D> {
    A(C),
    B(D),
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
                .map(|(_, &mut Inbox(ref mut rx_mpsc, _))| rx_mpsc));
            //
            let mut fut = outbox_rx_one.select2(inbox_rx_mpsc)
            .map_err(|_| Error::new(ErrorKind::Other, "TODO: error in select2 for sched!")); // TODO: change

            // Either::B needed to get one value out of this next scope,
            // so returning that vlaue was needed.
            // other returns are normal future workflow.
            match fut.poll() {
              //


  /*
type Rx_one = oneshot::Receiver<(
    WorkerResponse,
    AddrReqId
)>;

              Either A:
              count(vetor_canais_worker(hashmap)):
                Caso >0:
                  Retira esta tarefa do worker do hashmap
                  ordenar tb
                Caso = 0:
                  Mata o worker.*/

              // from Outbox
              //Ok(Async::Ready(Either::A(rx_one))) => {

              Ok(Async::Ready(Either::A(((WorkerResponseContent(wrk_response, addr_req_id),
                      _index, _vec_other_mpsc), _other_either)))) => {

                  Ok(Async::Ready(AorB::A((wrk_response, addr_req_id))))

                  //println!("parabéns, recebido rx_one: {:#?}", addr_req_id);
                  //Ok(Async::Ready(None)) // TODO
              },
              //
              Ok(Async::Ready(Either::B((((first, tail_stream),
                      _index, _vec_other_mpsc), _other_either)))) => {

                  println!("parabéns, recebido rx_mpsc: {:#?}", first);
                  let WorkerRequestContent(wrkmsg, tx_one, addr_req_id) = first.unwrap();

                  // the tail must be taken out of this scope, because
                  // there's no replace access into the self.inbox
                  Ok(Async::Ready(AorB::B((wrkmsg, tx_one, addr_req_id, tail_stream))))
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
            Ok(Async::Ready(AorB::A((wrk_response, addr_req_id)))) => {
                self.outbox
                    .iter_mut()
                    .map(|&mut Outbox(_, ref mut rx_one_hm)| rx_one_hm)
                    .find(|ref mut hm| hm.contains_key(&addr_req_id))
                    .unwrap()
                    .remove(&addr_req_id);

                let &mut Inbox(_, ref mut prev_oneshots) =
                    self.inbox.get_mut(&addr_req_id.0).unwrap();

                prev_oneshots.remove(&addr_req_id.1)
                    .unwrap()
                    .complete(WorkerResponseContent(wrk_response, addr_req_id));
                //TODO: "delete" worker if .len() == 0 (no more taks left for the worker, so he can be killed)

            }
            Ok(Async::Ready(AorB::B((wrk_msg, tx_one, addr_req_id, tail_stream)))) => {

              // reverse sorting
              self.outbox.sort_unstable_by(|a, b| b.cmp(a));

              let new_box_flag =  {
                if let Some(ref mut outbox) = self.outbox.iter().rev().next() {
                    if outbox.1.len() >= 10 {
                      true
                    } else {
                      false
                    }
                } else {
                  true
                }
              };

              if new_box_flag {
                  let (tx, rx) = mpsc::channel(10); // sched => worker mpsc
                  // The worker future (could be a machine)
                  let worker =  worker::Worker::new(rx)
                    .map(|item| ())
                    .map_err(|err| ());
                  // spawn the worke's future
                  tokio::spawn(worker);
                  // create a new outbox, that is created for each new worker
                  self.outbox.push(Outbox::new(tx));
              };

              let outbox = self.outbox.iter_mut().rev().next().unwrap();


              {
                // extract the inner variables
                let &mut Outbox(ref mut tx, ref mut hm) = outbox;
                //Creating a new oneshot channel.
                let (otx, orx) = oneshot::channel::<WorkerResponseContent>();

                //Creating a new WorkerRequestContent with desired values (work message, one shot to Worker, peer ID/Address)
                let wrk_req_cont =  WorkerRequestContent(
                  wrk_msg,
                  otx,
                  addr_req_id.clone());

                // unwrap is safe because there is a sufficient control over
                // channel usage; and receptor won't be dropped before transmissor
                tx.try_send(wrk_req_cont).unwrap();

                //Adding one_shot to outbox.
                hm.insert(addr_req_id.clone(), orx);
              }


              // get the Inbox related to the peer,
              let &mut Inbox(ref mut prev_rx_mpsc_sf, ref mut prev_oneshots) =
                self.inbox.get_mut(&addr_req_id.0).unwrap();
              // extract and replace the first future from the channel
              *prev_rx_mpsc_sf = tail_stream.into_future();
              if prev_oneshots.contains_key(&addr_req_id.1) {
                  println!("Error: colliding oneshot key");
                  panic!("TODO");
              }

              //
              prev_oneshots.insert(addr_req_id.1, tx_one);

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

