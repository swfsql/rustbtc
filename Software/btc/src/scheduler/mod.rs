//use errors::*;
//use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;

use std::net::SocketAddr;
//use std::thread;

use tokio;
//use tokio::io;
use futures;
use futures::sync::{mpsc, oneshot};
use futures::future::{select_all, Either};

use std::collections::HashMap;
//use std::iter::FromIterator;

use std::io::{Error, ErrorKind};
//use std::collections::BinaryHeap;
use std::cmp::Ordering;

use std::mem;
use std::borrow::BorrowMut;

pub mod worker;
pub mod commons;

use self::commons::{AddrReqId, RequestId, Rx_mpsc_sf, Rx_one, Tx_mpsc,
                    Tx_one, WorkerRequestContent,
                    WorkerResponseContent, Rx_peers};

struct Inbox(Rx_mpsc_sf, HashMap<RequestId, Tx_one>);
struct Outbox(Tx_mpsc, HashMap<AddrReqId, Rx_one>);

impl Outbox {
    fn new(tx: Tx_mpsc) -> Outbox {
        Outbox(tx, HashMap::new())
    }
}

impl Eq for Outbox {}

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

pub struct Scheduler {
    main_channel: mpsc::UnboundedReceiver<Rx_peers>,
    inbox: HashMap<SocketAddr, Inbox>,
    outbox: Vec<Outbox>,
}

enum AorB<C, D> {
    A(C),
    B(D),
}

impl Scheduler {
    pub fn new(rx: mpsc::UnboundedReceiver<Rx_peers>) -> Scheduler {
        Scheduler {
            main_channel: rx,
            inbox: HashMap::new(),
            outbox: vec![],
        }
    }
}

impl Inbox {
    fn new(first: Rx_mpsc_sf) -> Inbox {
        Inbox(first, HashMap::new())
    }
}

impl Future for Scheduler {
    type Item = ();
    type Error = Error;

    fn poll(&mut self) -> Poll<(), Error> {
        println!("Schedule poll called.");

        loop {
            match self.main_channel.poll() {
                Ok(Async::Ready(Some(Rx_peers(addr, first)))) => {
                    self.inbox.insert(addr, Inbox::new(first));
                },
                _ => {
                    break;
                }
            }
        };

        loop {
            if self.outbox.is_empty() {
                break;
            };
            let wrk_full_resp = {
                let poll = futures::future::select_all(
                        self.outbox
                            .iter_mut()
                            .flat_map(|&mut Outbox(_, ref mut rx_one_hm)|
                                rx_one_hm
                                    .iter_mut()
                                    .map(|(_, fut)| fut),
                            )
                    )
                    .map_err(|_| Error::new(ErrorKind::Other, "TODO: error in select(1) for sched!"))
                    .poll();
                if let Ok(Async::Ready((resp, _index, _vec))) = poll {
                    resp
                } else {
                    break;
                }
            };

            // replace the tail's first future (that will contain the next tail)
            // back into the channel (inbox)

            //Removing oneshot from hashmap from the worker who completed the task.
            let addr_req_id = {
                if let Ok(box WorkerResponseContent(ref wrk_response, ref addr_req_id)) = wrk_full_resp {
                    self.outbox.iter_mut().map(|&mut Outbox(_, ref mut rx_one_hm)| rx_one_hm)
                        .find(|ref mut hm| hm.contains_key(&addr_req_id)).unwrap().remove(&addr_req_id);
                        addr_req_id.clone()
                } else {
                    panic!("error from response.");
                    // TODO: unlist oneshot receiver on error,
                    // requires: same addr_req_id inside the error structure
                }
            };

            // Getting the oneshot channel to the peer
            let &mut Inbox(_, ref mut prev_oneshots) = self.inbox.get_mut(&addr_req_id.0).unwrap();

            // forwards the message to the peer
            prev_oneshots.remove(&addr_req_id.1).unwrap().send(wrk_full_resp);

            //**************************************************************
            //TODO: "delete" worker if .len() == 0 (no more taks left for the worker, so he can be killed)
            //**************************************************************
        };

        loop {
            if self.inbox.is_empty() {
                break;
            };
            let (first, tail_stream) = {
                let poll = futures::future::select_all(
                        self.inbox.iter_mut().map(|(_, &mut Inbox(ref mut rx_mpsc, _))| rx_mpsc),
                    )
                    .map_err(|_| Error::new(ErrorKind::Other, "TODO: error in select(2) for sched!"))
                    .poll();
                if let Ok(Async::Ready(((first, tail_stream), _index, _vec))) = poll {
                    (first, tail_stream)
                } else {
                    break;
                }
            };

            // reverse sorting, so its not needed
            self.outbox.sort_unstable_by(|a, b| b.cmp(a));

            //Creating a new oneshot channel.
            let (otx, orx) = oneshot::channel::<Result<Box<WorkerResponseContent>, _>>();

            // extract the old oneshot from the box, and replaces (in the box)
            // with the new one
            let mut first = first.unwrap();
            let (old_tx_one, addr_req_id) = {
                //first.unwrap().borrow_mut()
                let &mut WorkerRequestContent(ref _wrk_msg, ref mut tx_one, ref addr_req_id) = first.borrow_mut();
                (mem::replace(tx_one, otx), addr_req_id.clone())
            };

            let new_box_flag = {
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
                let worker = worker::Worker::new(rx).map(|item| ()).map_err(|err| ());
                // spawn the worke's future
                tokio::spawn(worker);
                // create a new outbox, that is created for each new worker
                self.outbox.push(Outbox::new(tx));
            };

            let outbox = self.outbox.iter_mut().rev().next().unwrap();

            {
                // extract the inner variables
                let &mut Outbox(ref mut tx, ref mut hm) = outbox;

                //Creating a new WorkerRequestContent with desired values (work message, one shot to Worker, peer ID/Address)
                //let wrk_req_cont = WorkerRequestContent(wrk_msg, otx, addr_req_id.clone());

                // unwrap is safe because there is a sufficient control over
                // channel usage; and receptor won't be dropped before transmissor
                tx.try_send(first).unwrap();

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
            prev_oneshots.insert(addr_req_id.1, old_tx_one);

        };

        Ok(Async::NotReady)
    }
}

