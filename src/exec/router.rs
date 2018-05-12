mod errors {
    error_chain!{}
}

//use errors::*;

use chrono::Utc;
use exec::commons;
use futures::sync::mpsc;
use rand;
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::sync::Arc;
use tokio;
use tokio::io;
use tokio::net::TcpStream;
use tokio::prelude::*;
//use rand::{Rng, thread_rng};
//use admin;
use codec;
use peer;
use futures::sync::{oneshot};

use codec::msgs::msg::commons::into_bytes::IntoBytes;
use codec::msgs::msg::commons::net_addr::NetAddr;
use codec::msgs::msg::commons::new_from_hex::NewFromHex;
use codec::msgs::msg::commons::var_str::VarStr;
//use codec::msgs::msg::commons::params::Network;
use codec::msgs::msg::header;
use codec::msgs::msg::header::Header;
use codec::msgs::msg::payload::version::Version;
use codec::msgs::msg::payload::Payload;
use codec::msgs::msg::Msg;
/*
use exec::commons::{RxMpsc, RouterRequest, RouterRequestContent, RouterRequestPriority,
                    RouterResponse, RouterResponseContent, SchedulerResponse};
*/

use std::time::*;
use tokio_timer::*;

struct Inbox(RxMpsc, Vec<Box<RouterRequestContent>>);

//
pub struct Router {
    inbox: Inbox,
    toolbox: commons::ToolBox,
}

impl Router {
    pub fn new(rx_mpsc: RxMpsc, toolbox: Arc<commons::ToolBox>) -> Router {
        Router {
            inbox: Inbox(rx_mpsc, vec![]),
            toolbox,
        }
    }
}

impl Future for Router {
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<(), io::Error> {
        d!("poll");

        // gets the router's requests receiver, and also the requests queue
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
                _ => panic!(ff!("Unexpected value for router polling on reader channel")),
            };
        }

        // give priority to requests with highest priority (last)
        reqs.sort_unstable();
        if let Some(req) = reqs.pop() {
            let mut req = *req;
            let RouterRequestContent(RouterRequestPriority(wrk_req, _req_pri), tx_one, addr) = req;
            let resp = match wrk_req {
                RouterRequest::Hello => {
                    // i!("Request received: {:#?}", wrk_req);
                    RouterResponse::Empty
                }
                RouterRequest::Wait { delay } => {
                    // i!("Request received: {:#?}", wrk_req);
                    let timer = Timer::default();
                    let sleep = timer.sleep(Duration::from_secs(delay));
                    sleep.wait().expect(&ff!());
                    RouterResponse::Empty
                }
                RouterRequest::ListPeers => {
                    // i!("Request received: {:#?}", &wrk_req);
                    let keys = self.toolbox
                        .peer_messenger
                        .lock()
                        .expect(&ff!())
                        .keys()
                        .cloned()
                        .collect();
                    //let msg = commons::PeerRequest::Dummy;
                    //tx.unbounded_send(Box::new(commons::RouterToPeerRequestAndPriority(msg, 100)));

                    RouterResponse::ListPeers(keys)
                }
                RouterRequest::PeerAdd {
                    addr,
                    wait_handhsake: _,
                    tx_sched,
                } => {
                    i!("PeerAdd Request received");

                    let self_addr = SocketAddr::new(
                        IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0x7f00, 1)),
                        8333,
                    ); // TODO get from toolbox
                    let version = 70013_i32;
                    let addr_trans = NetAddr::from_socket_addr(&addr);
                    let addr_recv = NetAddr::from_socket_addr(&self_addr);
                    let services = addr_trans.service;
                    let nonce = rand::random::<u64>(); // TODO record into peer and toolbox
                    let timestamp = Utc::now().timestamp();
                    let start_height = 0_i32; // maybe 1
                    let relay = Some(false);
                    let agent_bytes = b"/Rustbtc:0.0.1/";
                    let user_agent = VarStr::from_bytes(agent_bytes).expect(&ff!());

                    d!("version payload creating");
                    let version_pl = Version {
                        version,
                        services,
                        timestamp,
                        addr_recv,
                        addr_trans,
                        nonce,
                        user_agent,
                        start_height,
                        relay,
                    };
                    d!("version payload created");

                    let version_pl_raw = version_pl.into_bytes().expect(&ff!());

                    let version_header = Header {
                        network: header::network::Network::Main,
                        cmd: header::cmd::Cmd::Version,
                        payload_len: version_pl_raw.len() as i32,
                        payloadchk: Msg::chk(&version_pl_raw[..]).expect(&ff!()),
                    };
                    d!("version header created");

                    let version_msg = Msg {
                        header: version_header,
                        payload: Some(Payload::Version(version_pl)),
                    };

                    //d!("router:: PeerAdd Request received: {:#?}", &wrk_req);
                    match TcpStream::connect(&addr).wait() {
                        Ok(socket) => {
                            let (tx_peer, rx_peer) = mpsc::unbounded();
                            let (tx_toolbox, rx_toolbox) = mpsc::unbounded();
                            let peer_addr = socket.peer_addr().expect(&ff!());
                            {
                                d!("started sending rawmsg toolbox message to the new peer");
                                let boxed_binary = commons::RouterToPeerRequestAndPriority(
                                    commons::PeerRequest::RawMsg(
                                        version_msg.into_bytes().expect(&ff!()),
                                    ),
                                    100,
                                );
                                tx_toolbox
                                    .unbounded_send(Box::new(boxed_binary.clone()))
                                    .expect(&ff!());
                                d!("finished sending rawmsg toolbox message to the new peer");
                            }

                            d!("registering peer");
                            let actor_id = {
                                let tx_sched_unlocked = tx_sched.lock().expect(&ff!());

                                
                                let (otx, orx) = oneshot::channel::<Box<SchedulerResponse>>();
                                let sched_req_ctt = commons::MainToSchedRequestContent::Register(
                                    commons::RxPeers(peer_addr.clone(), rx_peer.into_future()),
                                    tx_toolbox,
                                    otx,
                                );

                                d!("before wait");
                                tx_sched_unlocked
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
                            let peer = peer::Peer::new(socket, tx_peer, tx_sched, rx_toolbox, actor_id);
                            {
                                //let mut messenger_unlocked = self.toolbox.peer_messenger.lock().unwrap();
                                //messenger_unlocked.insert(peer_addr, tx_toolbox);
                            }
                            let peer_machina = peer::machina::Machina::start(peer)
                                .map(|_| ())
                                .map_err(|_| ());
                            d!("spawning peer machina");
                            tokio::spawn(peer_machina);
                            d!("peer machina spawned");
                            RouterResponse::PeerAdd(Some(addr))
                        }
                        Err(_) => RouterResponse::PeerAdd(None),
                    }
                }
                RouterRequest::PeerRemove { addr } => {
                    d!("Router received PeerRemove command");
                    if let Some(tx) = self.toolbox
                        .peer_messenger
                        .lock()
                        .expect(&ff!())
                        .remove(&addr)
                    {
                        let msg = commons::PeerRequest::SelfRemove;
                        tx.unbounded_send(Box::new(commons::RouterToPeerRequestAndPriority(
                            msg, 255,
                        ))).expect(&ff!());
                        d!("Router sended SelfRemove command to Peer");
                        RouterResponse::Empty
                    } else {
                        RouterResponse::Empty
                    }
                }
                RouterRequest::MsgFromHex { send, binary } => {
                    //let msg = codec::msgs::msg::Msg::new_from_hex(&binary);
                    let msg = codec::msgs::msg::Msg::new(binary.iter());

                    //d!("Request received: {:#?}", &wrk_req);
                    d!("message from hex");
                    if send {
                        if let &Ok(ref _okmsg) = &msg {
                            for (_addr, tx) in
                                self.toolbox.peer_messenger.lock().expect(&ff!()).iter()
                            {
                                let boxed_binary = commons::RouterToPeerRequestAndPriority(
                                    commons::PeerRequest::RawMsg(binary.clone()),
                                    100,
                                );
                                tx.unbounded_send(Box::new(boxed_binary.clone()))
                                    .expect(&ff!());
                            }
                        }
                    };

                    RouterResponse::MsgFromHex(msg)
                }
                _ => {
                    // i!("Request received: {:#?}", wrk_req);
                    RouterResponse::Empty
                }
            };

            d!("response sending.");
            tx_one
                .send(Ok(Box::new(RouterResponseContent(resp, addr.clone()))))
                .expect(&ff!());
            d!("response sent.");
            task::current().notify();
        }
        d!("returning not ready (end).");
        Ok(Async::NotReady)
    }
}
