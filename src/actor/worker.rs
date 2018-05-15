mod errors {
    error_chain!{}
}

//use errors::*;


use actor::commons;
use futures::sync::mpsc;

use tokio;
use tokio::io;
use tokio::net::TcpStream;
use tokio::prelude::*;
//use rand::{Rng, thread_rng};
//use admin;
use codec;
use actor::peer;
use futures::sync::{oneshot};
use codec::msgs::msg::Msg;
use codec::msgs::msg;

use codec::msgs::msg::commons::new_from_hex::NewFromHex;
use codec::msgs::msg::commons::into_bytes::IntoBytes;

use actor::commons::channel_content::{WorkerRequest, WorkerRequestContent, WorkerRequestPriority,
                    WorkerResponse, WorkerResponseContent, SchedulerResponse, WorkerToRouterResponse,
                    WorkerToRouterRequestContent,WorkerToRouterRequest};
use actor::commons::{RxMpsc, TxMpscWorkerToRouter};

use std::time::*;
use tokio_timer::*;

struct Inbox(RxMpsc, Vec<Box<WorkerRequestContent>>);

pub struct Worker {
    inbox: Inbox,
    //toolbox: Arc<commons::ToolBox>,
    tx_router: TxMpscWorkerToRouter,
}

impl Worker {
    pub fn new(rx_mpsc: RxMpsc, tx_router: TxMpscWorkerToRouter) -> Worker {
        Worker {
            inbox: Inbox(rx_mpsc, vec![]),
            tx_router,
        }
    }
}

impl Future for Worker {
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<(), io::Error> {
        t!("poll");

        // gets the worker's requests receiver, and also the requests queue
        let Inbox(ref mut rec, ref mut reqs) = self.inbox;

        loop {
            d!("starting mpsc channel to scheduler loop");
            match rec.poll() {
                Ok(Async::Ready(Some(wrk_req))) => {
                    // brand new incomming requests
                    d!("{:?}", &wrk_req);
                    reqs.push(wrk_req);
                    d!("End of mpsc channel to scheduler loop (Ok Ready)");
                }
                Ok(Async::NotReady) => {
                    d!("not ready");
                    break
                },
                _ => {
                    d!("panic");
                    panic!(ff!("Unexpected value for worker polling on reader channel"));
                },
            };
        }
        d!("broke from loop");

        // give priority to requests with highest priority (last)
        reqs.sort_unstable();
        if let Some(req) = reqs.pop() {
            d!("inner if");
            let mut req = *req;
            let WorkerRequestContent(WorkerRequestPriority(wrk_req, _req_pri), tx_one, addr) = req;
            let resp = match wrk_req {
                WorkerRequest::Hello => {
                    d!("Request received: {:#?}", wrk_req);
                    WorkerResponse::Empty
                }
                WorkerRequest::Wait { delay } => {
                    d!("Request received: {:#?}", wrk_req);
                    let timer = Timer::default();
                    let sleep = timer.sleep(Duration::from_secs(delay));
                    sleep.wait().expect(&ff!());
                    WorkerResponse::Empty
                }
                WorkerRequest::ListPeers => {
                    d!("Request received: {:#?}", &wrk_req);

                    let (otx, orx) = oneshot::channel::<Box<WorkerToRouterResponse>>();

                    let wrk_to_router_req = WorkerToRouterRequest::ListPeers;
                    let wrk_to_router_req_content = WorkerToRouterRequestContent(wrk_to_router_req, Some(otx));
                    self.tx_router.unbounded_send(Box::new(wrk_to_router_req_content)).expect(&ff!());
                    d!("sent to router");

                    if let Ok(box WorkerToRouterResponse::ListPeers(peer_list)) = orx.wait() {
                         d!("todo: show peer list");
                        WorkerResponse::ListPeers(peer_list)
                        // i!("Peer list: {:?}", peer_list);
                    } else {
                        e!("Logic error");
                        panic!("logic error")
                    }

                }//
                WorkerRequest::PeerAdd {
                    addr,
                    wait_handshake: _,
                    tx_sched,
                } => {
                    d!("PeerAdd Request received");

                    let version_msg = Msg::new_version(addr);

                    //d!("worker:: PeerAdd Request received: {:#?}", &wrk_req);
                    match TcpStream::connect(&addr).wait() {
                        Ok(socket) => {
                            let (tx_peer, rx_peer) = mpsc::unbounded();
                            let (tx_router, rx_router) = mpsc::unbounded();
                            let peer_addr = socket.peer_addr().expect(&ff!());
                            {
                                d!("started sending rawmsg toolbox message to the new peer");
                                let boxed_binary = commons::channel_content::RouterToPeerRequestAndPriority(
                                    commons::channel_content::PeerRequest::HandShake(
                                        version_msg.into_bytes().expect(&ff!()),
                                    ),
                                    100,
                                );
                                tx_router
                                    .unbounded_send(Box::new(boxed_binary.clone()))
                                    .expect(&ff!());
                                d!("finished sending rawmsg toolbox message to the new peer");
                            }

                            d!("registering peer");
                            let actor_id = {
                                //let tx_sched_unlocked = tx_sched.lock().expect(&ff!());
                                let (otx, orx) = oneshot::channel::<Box<SchedulerResponse>>();
                                let sched_req_ctt = commons::channel_content::MainToSchedRequestContent::Register(
                                    peer_addr.clone(),
                                    rx_peer.into_future(),
                                    tx_router,
                                    otx,
                                );

                                d!("before wait");
                                tx_sched
                                    .unbounded_send(Box::new(sched_req_ctt))
                                    .expect(&ff!());//
                                let shot_back = orx.wait().expect(&ff!()); // TODO async
                                d!("after wait");
                                if let box SchedulerResponse::RegisterResponse(Ok(ref res_actor_id)) = shot_back {
                                    res_actor_id.clone()
                                } else {
                                    panic!("TODO: error when registering new peer");
                                }
                            };
                            d!("peer registered");
                            let peer = peer::Peer::new(socket, tx_peer, tx_sched, rx_router, actor_id);
                            {
                                //let mut messenger_unlocked = self.toolbox.peer_messenger.lock().unwrap();
                                //messenger_unlocked.insert(peer_addr, tx_router);
                            }
                            let peer_machina = peer::machina::Machina::start(peer)
                                .map(|_| ())
                                .map_err(|_| ());
                            d!("spawning peer machina");
                            tokio::spawn(peer_machina);
                            d!("peer machina spawned");
                            WorkerResponse::PeerAdd(Some(addr))
                        }
                        Err(_) => WorkerResponse::PeerAdd(None),
                    }
                },
                WorkerRequest::PeerRemove { actor_id } => {
                    d!("Worker received PeerRemove command");
                    let msg_to_peer = commons::channel_content::PeerRequest::SelfRemove;
                    let msg_to_peer_priority = commons::channel_content::RouterToPeerRequestAndPriority(msg_to_peer, 255);
                    let wrk_to_router_req = WorkerToRouterRequest::PeerRemove(
                            actor_id, 
                            Box::new(msg_to_peer_priority));
                    let (otx, orx) = oneshot::channel::<Box<WorkerToRouterResponse>>();
                    let wrk_to_router_req_content = WorkerToRouterRequestContent(wrk_to_router_req, Some(otx));
                    self.tx_router.unbounded_send(Box::new(wrk_to_router_req_content)).expect(&ff!());

                    if let Ok(box WorkerToRouterResponse::PeerRemove(status)) = orx.wait(){
                        WorkerResponse::PeerRemove(status)
                        // i!("Peer list: {:?}", peer_list);
                        // w!("todo: show peer list");
                    } else {
                        e!("Logic error");
                        panic!("logic error")
                    }

                },
                WorkerRequest::MsgFromHex { send, binary } => {
                    //let msg = codec::msgs::msg::Msg::new_from_hex(&binary);
                    let msg = Msg::new(binary.iter());
                              
                    //d!("Request received: {:#?}", &wrk_req);
                    d!("message from hex");
                    if send {
                        if let &Ok(ref _okmsg) = &msg {
                        let msg_to_peer = commons::channel_content::PeerRequest::Forward(binary.clone());
                        let msg_to_peer_priority = commons::channel_content::RouterToPeerRequestAndPriority(msg_to_peer, 100);
                        let wrk_to_router_req = WorkerToRouterRequest::MsgToAllPeers(
                                Box::new(msg_to_peer_priority));
                        //let (otx, orx) = oneshot::channel::<Box<WorkerToRouterResponse>>();
                        let wrk_to_router_req_content = WorkerToRouterRequestContent(wrk_to_router_req, None);
                        self.tx_router.unbounded_send(Box::new(wrk_to_router_req_content)).expect(&ff!());
                        // WorkerResponse::Empty
                        }
                    } else {
                        w!("Something wrong on msg to all peer");
                        // WorkerResponse::Empty
                    }
                    WorkerResponse::Empty

                },
                WorkerRequest::NewVersion{addr} => {
                    let version = Msg::new_version(addr);
                    WorkerResponse::Version(version)
                }
                WorkerRequest::NewVerack => {
                    let verack = Msg::new_verack();
                    WorkerResponse::Verack(verack)
                }
                _ => {
                    // i!("Request received: {:#?}", wrk_req);
                    WorkerResponse::Empty
                }
            };

            d!("response sending.");
            tx_one
                .send(Ok(Box::new(WorkerResponseContent(resp, addr.clone()))))
                .expect(&ff!());
            d!("response sent.");
            task::current().notify();
        }
        d!("returning not ready (end).");
        Ok(Async::NotReady)
    }
}
