mod errors {
    error_chain!{}
}

//use errors::*;

use chrono::Utc;
use actor::commons;
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
use actor::peer;
use futures::sync::{oneshot};
use std::collections::HashMap;

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

use actor::commons::{RxMpsc, TxMpscRouterToPeer, ActorId,
                    RxMpscWorkerToRouter, SchedulerResponse,
                    RxMpscSchedToRouter, WorkerToRouterRequest, WorkerToRouterRequestContent,
                    SchedToRouterRequestContent,WorkerToRouterResponse};

use std::time::*;
use tokio_timer::*;

pub struct Router {
    peer_messenger_reg: RxMpscSchedToRouter,
    peer_messenger: HashMap<ActorId, TxMpscRouterToPeer>,
    peer_messenger_addr: HashMap<ActorId, SocketAddr>,    
    rx_worker: RxMpscWorkerToRouter,
}

impl Router {
    pub fn new(peer_messenger_reg: RxMpscSchedToRouter, rx_worker: RxMpscWorkerToRouter) -> Router {
        Router {
            peer_messenger_reg,
            peer_messenger: HashMap::new(),
            peer_messenger_addr: HashMap::new(),
            rx_worker,
        }
    }
}

impl Future for Router {
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<(), io::Error> {
        d!("Router poll called.");
        loop {                
            match self.peer_messenger_reg.poll() {                
                Ok(Async::Ready(Some(box intention))) => match intention {
                    SchedToRouterRequestContent::Register(
                        actor_id,
                        addr,
                        tx_mpsc_peer,
                    ) => {
                        d!("Registering PeerMsgr from Actor Id {:?}", &actor_id);
                        self.peer_messenger.insert(actor_id, tx_mpsc_peer);
                        self.peer_messenger_addr.insert(actor_id, addr);
                    },
                    SchedToRouterRequestContent::Unregister(actor_id) => {
                        d!("Unregistering PeerMsgr from Actor Id {:?}", &actor_id);
                        self.peer_messenger.remove(&actor_id);
                        self.peer_messenger_addr.remove(&actor_id);
                    }
                },
                _ => {
                    d!("Undentified PeerMsgr");
                    break;
                }
            }
            task::current().notify();
        }

        match self.rx_worker.poll() {
            Ok(Async::Ready(Some(box WorkerToRouterRequestContent(req, Some(rx_one))))) => {
                task::current().notify();
                match req {
                    WorkerToRouterRequest::ListPeers => {
                        d!("List peers asked. TODO: implement it");
                        let hm_clone = self.peer_messenger_addr.clone();
                        let resp = WorkerToRouterResponse::ListPeers(hm_clone);
                        rx_one.send(Box::new(resp));
                    },
                    WorkerToRouterRequest::PeerRemove(actor_id, msg_to_peer_priority) => {

                        if let Some(tx) = self.peer_messenger.remove(&actor_id) {
                            d!("Router sent SelfRemove command to Peer");
                            tx.send(msg_to_peer_priority);
                        } else {
                            e!("Error when Deleting peer");
                        }
                        let resp = WorkerToRouterResponse::PeerRemove(true);
                        rx_one.send(Box::new(resp));
                    },
                    
                    _ => {
                        w!("Router Logic error");
                    },
                };
            },
            Ok(Async::Ready(Some(box WorkerToRouterRequestContent(req, _)))) => {
                task::current().notify();
                match req {
                    WorkerToRouterRequest::MsgToPeer(actor_id, peer_req) => {
                        if let Some(chn) = self.peer_messenger.get(&actor_id) {
                            chn.unbounded_send(peer_req);
                        };
                    },
                    WorkerToRouterRequest::MsgToAllPeers(ref msg_to_peer_priority) => {
                        for (_actor_id, tx) in self.peer_messenger.iter() {
                            tx.unbounded_send(msg_to_peer_priority.clone());
                        }
                    },

                    _ => {
                        w!("Router Logic error");
                    }
                };
            }
            _ => {
                i!("Router poll result did not Ok(Ready).");
            },
        }

        Ok(Async::NotReady)
    }
}
