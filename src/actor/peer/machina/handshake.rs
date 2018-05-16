extern crate tokio;
#[macro_use]
//use error_chain;
pub mod errors {
    error_chain!{
        foreign_links {
            Io(::std::io::Error);
        }
    }
}
use errors::*;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use state_machine_future::RentToOwn;
use tokio::prelude::*;
//use std::net::SocketAddr;

use codec::msgs::msg::commons::into_bytes::IntoBytes;

use actor::peer::Peer;
use codec::msgs::msg::payload::Payload;
use codec::msgs::msg::Msg;

use actor::commons::channel_content::{
    AddrReqId, PeerRequest, WorkerRequest, WorkerRequestContent, WorkerRequestPriority,
    WorkerResponse, WorkerResponseContent,
};

use futures::sync::oneshot;

type OptVersion = Option<Msg>;
type OptVerack = Option<Msg>;
type RdyVersion = Msg;

#[derive(StateMachineFuture)]
pub enum Machina {
    #[state_machine_future(start, transitions(A, B))]
    Start(Peer),

    #[state_machine_future(transitions(A, Ready))]
    A(Peer, OptVersion, OptVerack),

    #[state_machine_future(transitions(Ready))]
    B(Peer, RdyVersion),

    #[state_machine_future(ready)]
    Ready((Peer, RdyVersion)),

    #[state_machine_future(error)]
    Error(errors::Error),
}

// macro_rules! ok_some {
//     ($e:expr) => {
//         match $e {
//             // Ok(Async::Ready(t)) => Some(t),
//             Ok(Async::Ready(Some(t))) => Some(t),
//             Ok(Async::NotReady) => None,
//             Ok(Async::Ready(None)) => bail!("aborted"),
//             Err(e) => bail!("Error on ok_ready: {:?}", e), //Err(From::from(e)),
//         }
//     };
// }

// defmac!(worker_request mut state_peer, wr, priority => {
//     let wrp = WorkerRequestPriority(wr, priority);
//     let (otx, orx) = oneshot::channel::<Result<Box<WorkerResponseContent>>>();
//     let actor_id = state_peer.actor_id;
//     let addr = AddrReqId(actor_id, state_peer.next_request_counter());
//     let wrc = WorkerRequestContent(wrp, otx, addr);
//     state_peer._tx_req.unbounded_send(Box::new(wrc))
//         .expect(&ff!());
//     (state_peer, orx.and_then(|i| Ok(i.expect(&ff!()).0)))
// });

impl PollMachina for Machina {
    fn poll_start<'a>(state: &'a mut RentToOwn<'a, Start>) -> Poll<AfterStart, errors::Error> {
        // if let Ok(Async::Ready(Some(box RouterToPeerRequestAndPriority(peer_req, _priority)))) =  peer.0.rx_router.poll()

        // check router (we are introducing)
d!("1");
        if let Some(req) = ok_some!(state.0.rx_router.poll()) {
            let state = state.take();
            let mut peer = state.0;
d!("2");
            if let PeerRequest::HandShake(raw) = req.0 {
                // forward version (introduction)
                peer.codec.buffer(raw.as_slice());
                peer.codec.poll_flush()?;
d!("3 - sent handshake stuff");
                let next = A(peer, None, None);
                transition!(next)
            } else {
                bail!("wrong message: {:?}", req.0);
            }
        }
d!("4");

        // check socket (they are introducing)
        if let Some(msg) = ok_some!(state.0.codec.poll()) {
            let state = state.take();
            let mut peer = state.0;

d!("5");
            let other_ver = match &msg.payload {
                Some(Payload::Version(ver)) => ver,
                _ => bail!("Wrong message"),
            };
d!("6");
            // asks for version and verack for workers
            let (mut peer, orx_ver) = worker_request!(
                peer,
                WorkerRequest::NewVersion {
                    addr: SocketAddr::from(other_ver.addr_recv.clone())
                },
                100
            );
d!("7");
            let (mut peer, orx_verack) = worker_request!(peer, WorkerRequest::NewVerack, 100);
            if let (WorkerResponse::Version(ver), WorkerResponse::Verack(verack)) =
                orx_ver.join(orx_verack).wait().expect(&ff!())
            {
                // sends version and verack
d!("8");
                peer.codec.buffer(&ver.into_bytes().expect(&ff!()));
d!("9");
                peer.codec.buffer(&verack.into_bytes().expect(&ff!()));
d!("10");
                peer.codec.poll_flush()?;
                let next = B(peer, msg.clone());
d!("11 - sent handshake stuff");
                transition!(next)
            } else {
d!("12");
                bail!("Error on Worker Response");
            }
        }
        return Ok(Async::NotReady);
    }

    fn poll_a<'a>(state: &'a mut RentToOwn<'a, A>) -> Poll<AfterA, errors::Error> {
        // check socket (they are sending version and/or verack)
        if let Some(msg) = ok_some!(state.0.codec.poll()) {
            let mut state = state.take();
            let mut peer = state.0;

            match &msg.payload {
                Some(Payload::Version(_)) => {
                    ensure!(state.1.is_none(), "Version already recieved");
                    state.1 = Some(msg);
d!("13");
                }
                Some(Payload::Verack) => {
                    ensure!(state.2.is_none(), "Verack already received");
                    state.2 = Some(msg);
d!("14");
                }
                _ => bail!("Wrong message"),
            }

            match (state.1, state.2) {
                (Some(ver), Some(_verack)) => {
                    let (mut peer, orx_verack) =
                        worker_request!(peer, WorkerRequest::NewVerack, 100);
d!("15");
                    if let WorkerResponse::Verack(verack) = orx_verack.wait().expect(&ff!()) {
d!("16");
                        peer.codec.buffer(&verack.into_bytes().expect(&ff!()));
                        peer.codec.poll_flush()?;
d!("17");
                        let next = Ready((peer, ver)); //
d!("18");                        
                        transition!(next)
                        
                    } else {
d!("19");
                        bail!("Wrong message")
                    }
                }
                (st1, st2) => {
                    let next = A(peer, st1, st2);
d!("20");
                    transition!(next)

                }
            }
        }
        return Ok(Async::NotReady);
    }

    fn poll_b<'a>(state: &'a mut RentToOwn<'a, B>) -> Poll<AfterB, errors::Error> {
        // check socket (they are sending version and/or verack)
        if let Some(msg) = ok_some!(state.0.codec.poll()) {
d!("21");
            let state = state.take();
            let peer = state.0;
            let ver = state.1;
d!("22");
            if let Some(Payload::Verack) = &msg.payload {
d!("23");
                let next = Ready((peer, ver));
                transition!(next)
            } else {
d!("24");
                bail!("Wrong message");
            }
        }
        return Ok(Async::NotReady);
    }
}
