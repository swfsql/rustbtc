use exec;
use exec::commons::{AddrReqId, RequestId, RxMpscSf, RxOne, RxPeers, TxMpsc, TxOne,
                    WorkerRequestContent, WorkerResponseContent};
use futures;
use futures::sync::{mpsc, oneshot};
use std::borrow::BorrowMut;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::mem;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio;
use tokio::prelude::*;

struct Inbox(RxMpscSf, HashMap<RequestId, TxOne>);
struct Outbox(TxMpsc, HashMap<AddrReqId, RxOne>);

impl Outbox {
    fn new(tx: TxMpsc) -> Outbox {
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
    main_channel: exec::commons::RxMpscMainToSched,
    inbox: HashMap<SocketAddr, Inbox>,
    outbox: Vec<Outbox>,
    workers_max_tasks: usize,
    toolbox: Arc<exec::commons::ToolBox>,
}

impl Scheduler {
    pub fn new(rx: exec::commons::RxMpscMainToSched, workers_max_tasks: usize) -> Scheduler {
        Scheduler {
            main_channel: rx,
            inbox: HashMap::new(),
            outbox: vec![],
            workers_max_tasks,
            toolbox: Arc::new(exec::commons::ToolBox::new()),
        }
    }
}

impl Inbox {
    fn new(first: RxMpscSf) -> Inbox {
        Inbox(first, HashMap::new())
    }
}

impl Future for Scheduler {
    type Item = ();
    type Error = Error;

    fn poll(&mut self) -> Poll<(), Error> {
        d!("Schedule poll called.");
        loop {
            match self.main_channel.poll() {
                Ok(Async::Ready(Some(box intention))) => match intention {
                    exec::commons::MainToSchedRequestContent::Register(
                        RxPeers(addr, first),
                        tx_mpsc_peer,
                    ) => {
                        self.inbox.insert(addr, Inbox::new(first));
                        self.toolbox
                            .peer_messenger
                            .lock()
                            .unwrap()
                            .insert(addr, tx_mpsc_peer);
                    }
                    exec::commons::MainToSchedRequestContent::Unregister(addr) => {
                        d!("Unregistering Inbox for addr {:?}", &addr);
                        self.inbox.remove(&addr);
                    }
                },
                _ => {
                    break;
                }
            }
            task::current().notify();
        }

        loop {
            if let Some(first_outbox) = self.outbox.iter().next() {
                if first_outbox.1.is_empty() {
                    break;
                }
            } else {
                break;
            };
            let wrk_full_resp = {
                let poll = futures::future::select_all(self.outbox.iter_mut().flat_map(
                    |&mut Outbox(_, ref mut rx_one_hm)| rx_one_hm.iter_mut().map(|(_, fut)| fut),
                )).map_err(|_| {
                    Error::new(ErrorKind::Other, "TODO: error in select(1) for sched!")
                })
                    .poll();
                if let Ok(Async::Ready((resp, _index, _vec))) = poll {
                    resp
                } else if let Err(e) = poll {
                    panic!("Outbox Error: \n{:?}", e);
                } else {
                    break;
                }
            };

            // replace the tail's first future (that will contain the next tail)
            // back into the channel (inbox)

            d!("Removing oneshot from hashmap from the worker who completed the task.");
            let addr_req_id = {
                if let Ok(box WorkerResponseContent(ref _wrk_response, ref addr_req_id)) =
                    wrk_full_resp
                {
                    self.outbox
                        .iter_mut()
                        .map(|&mut Outbox(_, ref mut rx_one_hm)| rx_one_hm)
                        .find(|ref mut hm| hm.contains_key(&addr_req_id))
                        .unwrap()
                        .remove(&addr_req_id);
                    addr_req_id.clone()
                } else {
                    e!("No channel oneshot to be removed on the scheduler's outbox");
                    panic!("error from response.");
                    // TODO: unlist oneshot receiver on error,
                    // requires: same addr_req_id inside the error structure
                }
            };
            d!("Removed oneshot from hashmap from the worker who completed the task.");

            // Getting the oneshot channel to the peer
            let &mut Inbox(_, ref mut prev_oneshots) = self.inbox.get_mut(&addr_req_id.0).unwrap();

            // forwards the message to the peer
            prev_oneshots
                .remove(&addr_req_id.1)
                .unwrap()
                .send(wrk_full_resp)
                .unwrap();

            //**************************************************************
            //TODO: "delete" worker if .len() == 0 (no more taks left for the worker, so he can be killed)
            //**************************************************************
            task::current().notify();
        }

        loop {
            if self.inbox.is_empty() {
                break;
            };
            let (first, tail_stream) = {
                let poll = futures::future::select_all(
                    self.inbox
                        .iter_mut()
                        .map(|(_, &mut Inbox(ref mut rx_mpsc, _))| rx_mpsc),
                ).map_err(|_| {
                    Error::new(ErrorKind::Other, "TODO: error in select(2) for sched!")
                })
                    .poll();
                if let Ok(Async::Ready(((first, tail_stream), _index, _vec))) = poll {
                    d!("received new request to be forwarded\n {:#?}", &first);
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
            d!("Before unwrapping first");
            let mut first = first.unwrap();
            let (old_tx_one, addr_req_id) = {
                //first.unwrap().borrow_mut()
                let &mut WorkerRequestContent(ref _wrk_msg, ref mut tx_one, ref addr_req_id) =
                    first.borrow_mut();
                (mem::replace(tx_one, otx), addr_req_id.clone())
            };

            let new_box_flag = {
                if let Some(ref mut outbox) = self.outbox.iter().rev().next() {
                    if outbox.1.len() >= self.workers_max_tasks {
                        true
                    } else {
                        false
                    }
                } else {
                    true
                }
            };

            if new_box_flag {
                let (tx, rx) = mpsc::unbounded(); // sched => worker mpsc
                                                  // The worker future (could be a machine)
                let worker = exec::worker::Worker::new(rx, self.toolbox.clone())
                    .map(|_item| ())
                    .map_err(|_err| ());
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
                tx.unbounded_send(first).unwrap();

                //Adding one_shot to outbox.
                hm.insert(addr_req_id.clone(), orx);
            }

            // get the Inbox related to the peer,
            let &mut Inbox(ref mut prev_rx_mpsc_sf, ref mut prev_oneshots) =
                self.inbox.get_mut(&addr_req_id.0).unwrap();
            // extract and replace the first future from the channel
            *prev_rx_mpsc_sf = tail_stream.into_future();
            if prev_oneshots.contains_key(&addr_req_id.1) {
                e!("Error: colliding oneshot key on inserting inbox");
                panic!("TODO");
            }

            //
            prev_oneshots.insert(addr_req_id.1, old_tx_one);
            task::current().notify();
        }

        Ok(Async::NotReady)
    }
}
