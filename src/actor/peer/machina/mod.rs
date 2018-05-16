pub mod errors {
    error_chain!{
        foreign_links {
            Io(::std::io::Error);
            HandshakeError(::actor::peer::machina::handshake::errors::Error);
        }
    }
}
pub use errors::*;

// first implementation here
//   |        conflicting implementation for `actor::peer::machina::errors::Error`



use std;

use tokio::prelude::*;

use state_machine_future::RentToOwn;

use actor::peer::Peer;
use codec::msgs::msg::commons::into_bytes::IntoBytes;
// use actor::peer::machina::handshake::Handshake;



//use structopt::StructOpt;

//use actor::commons::{AddrReqId, RequestId, RxMpscSf, RxOne, TxMpsc,
//                   TxOne, WorkerRequest, WorkerRequestContent, WorkerRequestPriority,
//                  WorkerResponseContent, RxPeers};

use actor::commons::channel_content::{
    MainToSchedRequestContent, PeerRequest, RouterToPeerRequestAndPriority,WorkerResponse,
    WorkerRequestPriority,AddrReqId,WorkerResponseContent,WorkerRequest,WorkerRequestContent
};
use actor::commons::RxOne;

//use futures::sync::{mpsc, oneshot};
use futures::sync::{oneshot};
//use futures;
//use std::io::{Error, ErrorKind};

//use env_logger::LogBuilder;
//#[macro_use]
//use macros;
pub mod handshake;
// use btc;
use codec;
use hex::ToHex;

//use macros;
/*
use macros::*;
*/



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


macro_rules! ok_some {
    ($e:expr) => {
        match $e {
            // Ok(Async::Ready(t)) => Some(t),
            Ok(Async::Ready(Some(t))) => Some(t),
            Ok(Async::NotReady) => None,
            Ok(Async::Ready(None)) => bail!("aborted"),
            Err(e) => bail!("Error on ok_ready: {:?}", e), //Err(From::from(e)),
        }
    };
}

#[derive(StateMachineFuture)]
pub enum Machina {
    #[state_machine_future(start, transitions(Standby))]
    Handshake(handshake::MachinaFuture),

    #[state_machine_future(transitions(SimpleWait, SelfRemove, End))]
    Standby(Peer),

    #[state_machine_future(transitions(Standby))]
    SimpleWait(Peer, RxOne),

    #[state_machine_future(transitions(End))]
    SelfRemove(Peer),

    #[state_machine_future(ready)]
    End(Peer),

    #[state_machine_future(error)]
    MachinaError(errors::Error),
}

impl PollMachina for Machina {
    fn poll_handshake<'a>(
        state: &'a mut RentToOwn<'a, Handshake>,
    ) -> Poll<AfterHandshake, errors::Error> {
        //peer.0.lines.buffer(b"WELCOME\r\n");
        //let _ = peer.0.lines.poll_flush()?;
        //let _ = peer.0.lines.poll_flush()?; // to make sure
        //d!("sent WELCOME");

        d!();
        let (mut peer, mut version_msg) = try_ready!(state.0.poll());
d!();

        // let (mut peer, mut version_msg) = match ok_some!(state.0.poll()) {
        //     Some(tuple) => tuple,
        //     None => return Ok(Async::NotReady),
        // };

        // let peer = peer.take();
        peer.version = Some(version_msg);

        let (mut peer, orx_gh) = worker_request!(peer, WorkerRequest::NewGetHeaders, 100);
d!();
        if let WorkerResponse::GetHeaders(gh) = orx_gh.wait().expect(&ff!()) {
d!("{:?}", gh);
d!("{:?}", gh.into_bytes());
            // sends get_headers
            peer.codec.buffer(&gh.into_bytes().expect(&ff!()));
            peer.codec.poll_flush()?;
d!("sent getheaders");
            let next = Standby(peer);
            transition!(next)
        } else {
d!();
            bail!("Error on Worker Response");
        };
        // transition!(Standby(peer.take().0))
    }

    fn poll_standby<'a>(
        peer: &'a mut RentToOwn<'a, Standby>,
    ) -> Poll<AfterStandby, errors::Error> {
        d!("poll standby");
        /*
        defmac!(prepare_transition mut state_peer, wr, priority => {
            let wrp = WorkerRequestPriority(wr, priority);
            let (otx, orx) = oneshot::channel::<Result<Box<WorkerResponseContent>, _>>();
            let skt = state_peer.lines.socket.peer_addr().unwrap();
            let addr = AddrReqId(skt, state_peer.next_request_counter());
            let wrc = WorkerRequestContent(wrp, otx, addr);
            state_peer.tx_req.unbounded_send(Box::new(wrc)).unwrap();
            (state_peer, orx)
        });
        */

        peer.0.poll_ignored();

        loop {
            if let Ok(Async::Ready(Some(box RouterToPeerRequestAndPriority(peer_req, _priority)))) =
                peer.0.rx_router.poll()
            {
                match peer_req {
                    PeerRequest::Dummy => {
                        i!("received dummy command, read on standby");
                    }
                    PeerRequest::SelfRemove => {
                        w!("received selfRemove command");
                        // TODO: WAIT FOR TRASH GET EMPTY

                        let peer = peer.take();
                        let next = SelfRemove(peer.0); //Calling this simplewait???
                        transition!(next);
                    }
                    PeerRequest::Forward(raw_msg) => {
                        let bytes = codec::msgs::msg::commons::bytes::Bytes::new(raw_msg.clone());
                        i!("received RawMsg command:\n{}{:?}", &raw_msg.to_hex(), bytes);
                        peer.0.codec.buffer(&raw_msg);
                        let _ = peer.0.codec.poll_flush()?;
                    }
                    PeerRequest::HandShake(_raw_msg) => {}
                }
            } else {
                break;
            }
        }

        d!("Before polling codec");
        while let Some(_msg) = try_ready!(peer.0.codec.poll()) {
            i!("Message received (but ignored for now):");


            // TODO: verify message type and transition and stuff
        }
        // Err(std::io::Error::new(std::io::ErrorKind::ConnectionAborted, "Peer connection aborted."))
        panic!("Peer connection aborted.");
    }

    fn poll_simple_wait<'a>(
        state: &'a mut RentToOwn<'a, SimpleWait>,
    ) -> Poll<AfterSimpleWait, errors::Error> {
        d!("SimpleWait pool called");

        let resp;
        match state.1.poll() {
            Ok(Async::Ready(fresp)) => {
                resp = fresp;
            }
            Ok(Async::NotReady) => {
                return Ok(Async::NotReady);
            }
            Err(e) => {
                panic!("Canceled scheduler response. \n{:#?}", e);
            }
        };

        let state = state.take();
        let mut peer = state.0;
        let _orx = state.1;

        peer.codec.buffer(format!("{:#?}", &resp).as_bytes());
        let _ = peer.codec.poll_flush()?;
        let _ = peer.codec.poll_flush()?; // to make sure
        i!("{:#?}", &resp);

        //orx.take();
        let next = Standby(peer);
        transition!(next)
    }

    fn poll_self_remove<'a>(
        state: &'a mut RentToOwn<'a, SelfRemove>,
    ) -> Poll<AfterSelfRemove, errors::Error> {
        d!("Entered self_remove state!");
        state.0.poll_ignored();
        if state.0.rx_ignored.len() > 0 {
            return Ok(Async::NotReady);
        }
        d!("Dumped all trash responses!");
        let state = state.take();
        // let addr = state.0.codec.socket.peer_addr().expect(&ff!());
        let actor_id = state.0.actor_id;
        let msg = MainToSchedRequestContent::Unregister(actor_id);
        state
            .0
            .tx_sched
            .unbounded_send(Box::new(msg))
            .expect(&ff!());
        let next = End(state.0); //Calling this simplewait???
        transition!(next);
    }
} //
