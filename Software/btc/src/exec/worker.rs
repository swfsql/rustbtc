use tokio::net::{TcpStream};
use tokio::prelude::*;
use tokio::io;
use tokio;
use futures::sync::{mpsc};
use std::sync::{Arc};

use exec::commons;
//use admin;
use peer;
use codec;
use codec::msgs::msg::commons::new_from_hex::NewFromHex;

use exec::commons::{RxMpsc, WorkerRequestContent, WorkerRequest, WorkerResponse, WorkerRequestPriority, WorkerResponseContent, MainToSchedRequestContent};


use tokio_timer::*;
use std::time::*;

struct Inbox(RxMpsc, Vec<Box<WorkerRequestContent>>);

pub struct Worker {
    inbox: Inbox,
    toolbox: Arc<commons::ToolBox>,
}

impl Worker {
    pub fn new(rx_mpsc: RxMpsc, toolbox: Arc<commons::ToolBox>) -> Worker {
        Worker {
            inbox: Inbox(rx_mpsc, vec![]),
            toolbox,
        }
    }
}

impl Future for Worker {
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<(), io::Error> {

        d!("poll");

        // gets the worker's requests receiver, and also the requests queue
        let Inbox(ref mut rec, ref mut reqs) = self.inbox;

        loop {
            d!("starting mpsc channel to scheduler loop");
            match rec.poll() {
                Ok(Async::Ready(Some(wrk_req))) => {
                    // brand new incomming requests
                    reqs.push(wrk_req);
                    d!("End of mpsc channel to scheduler loop (Ok Ready)");
                }
                Ok(Async::NotReady) => break,
                _ => panic!("Unexpected value for worker polling on reader channel"),
            };
        }

        // give priority to requests with highest priority (last)
        reqs.sort_unstable();
        if let Some(req) = reqs.pop() {
            let mut req = *req;
            let WorkerRequestContent(
                WorkerRequestPriority(wrk_req, _req_pri),
                tx_one,
                addr) = req;
            let resp = match wrk_req {
                WorkerRequest::Hello => {
                    i!("Request received: {:#?}", wrk_req);
                    WorkerResponse::Empty
                },
                WorkerRequest::Wait{delay} => {

                    i!("Request received: {:#?}", wrk_req);
                    let timer = Timer::default();
                    let sleep = timer.sleep(Duration::from_secs(delay));
                    sleep.wait().unwrap();
                    WorkerResponse::Empty
                },
                WorkerRequest::ListPeers => {

                    i!("Request received: {:#?}", &wrk_req);
                    let keys = self.toolbox.peer_messenger
                        .lock()
                        .unwrap()
                        .keys()
                        .cloned()
                        .collect();
                        //let msg = commons::PeerRequest::Dummy;
                        //tx.unbounded_send(Box::new(commons::WorkerToPeerRequestAndPriority(msg, 100)));

                    WorkerResponse::ListPeers(keys)
                },
                WorkerRequest::PeerAdd{addr, wait_handhsake: _, tx_sched} => {
                    //d!("worker:: PeerAdd Request received: {:#?}", &wrk_req);
                    match TcpStream::connect(&addr).wait() {
                        Ok(socket) => {
                            let (tx_peer, rx_peer) = mpsc::unbounded();
                            let (tx_toolbox, rx_toolbox) = mpsc::unbounded();
                            let peer_addr = socket.peer_addr().unwrap();
                            {
                                let tx_sched_unlocked = tx_sched.lock().unwrap();

                                let sched_req_ctt =
                                commons::MainToSchedRequestContent::Register(
                                    commons::RxPeers(
                                        peer_addr.clone(),
                                        rx_peer.into_future()
                                    ),
                                    tx_toolbox);

                                tx_sched_unlocked.unbounded_send(Box::new(sched_req_ctt)).unwrap();
                            }
                            let peer = peer::Peer::new(socket, tx_peer, tx_sched, rx_toolbox);
                            {
                                //let mut messenger_unlocked = self.toolbox.peer_messenger.lock().unwrap();
                                //messenger_unlocked.insert(peer_addr, tx_toolbox);
                            }
                            let peer_machina = peer::machina::Machina::start(peer).map(|_| ()).map_err(|_| ());
                            tokio::spawn(peer_machina);
                            WorkerResponse::PeerAdd(Some(addr))
                        },
                        Err(_) => {WorkerResponse::PeerAdd(None)},
                    }
                },
                WorkerRequest::PeerRemove{addr} => {
                    d!("Worker received PeerRemove command");
                    if let Some(tx) = self.toolbox.peer_messenger.lock().unwrap().remove(&addr) {

                        let msg = commons::PeerRequest::SelfRemove;
                        tx.unbounded_send(Box::new(commons::WorkerToPeerRequestAndPriority(msg, 255))).unwrap();
                        d!("Worker sended SelfRemove command to Peer");
                        WorkerResponse::Empty
                    } else {
                        WorkerResponse::Empty
                    }
                },
                WorkerRequest::MsgFromHex{send, binary} => {
                    //let msg = codec::msgs::msg::Msg::new_from_hex(&binary);
                    let msg = codec::msgs::msg::Msg::new(
                        binary.clone().into_iter().by_ref());

                    //d!("Request received: {:#?}", &wrk_req);
                    d!("message from hex");
                    if send {
                        if let &Ok(ref _okmsg) = &msg {
                            for (_addr, tx) in self.toolbox.peer_messenger.lock().unwrap().iter() {
                                let boxed_binary =
                                        commons::WorkerToPeerRequestAndPriority(
                                            commons::PeerRequest::RawMsg(binary.clone()), 100);
                                tx.unbounded_send(Box::new(boxed_binary.clone())).unwrap();
                            }
                        }
                    };

                    WorkerResponse::MsgFromHex(msg)
                },
                _ => {
                    i!("Request received: {:#?}", wrk_req);
                    WorkerResponse::Empty
                },
            };

            d!("response sending.");
            tx_one.send(Ok(Box::new(WorkerResponseContent(resp, addr.clone())))).unwrap();
            d!("response sent.");
            task::current().notify();
        }
        d!("returning not ready (end).");
        Ok(Async::NotReady)
    }
}
