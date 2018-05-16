
use error_chain;
pub mod errors {
    error_chain!{
        foreign_links {
            Io(::std::io::Error);
        }
    }
}
use errors::*;


use std;

use tokio::prelude::*;

use state_machine_future::RentToOwn;

use actor::admin::args;
use actor::admin::Peer;

use structopt::StructOpt;

//use actor::commons::{AddrReqId, RequestId, RxMpscSf, RxOne, TxMpsc,
//                   TxOne, WorkerRequest, WorkerRequestContent, WorkerRequestPriority,
//                  WorkerResponseContent, RxPeers};

use actor::commons::channel_content::{
    AddrReqId, MainToSchedRequestContent, WorkerRequest, WorkerRequestContent,
    WorkerRequestPriority, WorkerResponseContent,
};

use actor::commons::RxOne;

//use futures::sync::{mpsc, oneshot};
use futures::sync::oneshot;
//use futures;
//use std::io::{Error, ErrorKind};

//use env_logger::LogBuilder;
//#[macro_use]
//use macros;

//use codec;
//use hex::ToHex;

//use macros;
/*
use macros::*;
*/

#[derive(StateMachineFuture)]
pub enum Machina {
    #[state_machine_future(start, transitions(Standby))]
    Welcome(Peer),

    #[state_machine_future(transitions(Execution, SimpleWait, SelfRemove, End))]
    Standby(Peer),

    #[state_machine_future(transitions(Standby))]
    SimpleWait(Peer, RxOne),

    #[state_machine_future(transitions(End))]
    SelfRemove(Peer),

    #[state_machine_future(transitions(End))]
    Execution(Peer),

    #[state_machine_future(ready)]
    End(Peer),

    #[state_machine_future(error)]
    MachinaError(errors::Error),
}

impl PollMachina for Machina {
    fn poll_welcome<'a>(
        state: &'a mut RentToOwn<'a, Welcome>,
    ) -> Poll<AfterWelcome, errors::Error> {
        state.0.codec.buffer(b"WELCOME\r\n");
        let _ = state.0.codec.poll_flush()?;
        let _ = state.0.codec.poll_flush()?; // to make sure
        d!("sent WELCOME");

        transition!(Standby(state.take().0))
    }

    fn poll_standby<'a>(
        state: &'a mut RentToOwn<'a, Standby>,
    ) -> Poll<AfterStandby, errors::Error> {
        d!("poll standby");


        state.0.poll_ignored();

        // possibility for admin to listen to the toolbox peer_messenger; unused for now
        /*
        loop {
            if let Ok(Async::Ready(Some(box RouterToPeerRequestAndPriority(peer_req, priority)))) = peer.0.rx_router.poll() {
                match peer_req {

                    PeerRequest::Dummy => {
                        i!("received dummy command, read on standby");
                    },
                    PeerRequest::SelfRemove => {
                        w!("received selfRemove command");
                        // TODO: WAIT FOR TRASH GET EMPTY

                        let peer = peer.take();
                        let next = SelfRemove(peer.0); //Calling this simplewait???
                        transition!(next);

                    },
                    PeerRequest::RawMsg(raw_msg) => {
                        let bytes = codec::msgs::msg::commons::bytes::Bytes::new(raw_msg.clone());
                        i!("received RawMsg command:\n{}{:?}", raw_msg.to_hex(), bytes);
                    },
                    _ => {i!("loop de recibo inner");
                    },
                }
            } else {
                break;
            }
        }
        */

        while let Some(msg) = try_ready!(state.0.codec.poll()) {
            let msg = String::from_utf8(msg.to_vec()).expect(&ff!());

            // The first element can be empty,
            // since the arg parser will consider
            // the first one as the the program's name
            let arg_msg = format!(" {}", &msg);

            let matches = args::AdminCmd::clap().get_matches_from_safe(arg_msg.split(' '));
            match matches {
                Err(e) => {
                    // match e.kind {
                    //     clap::ErrorKind::
                    //     ErrorKind::HelpDisplayed or ErrorKind::VersionDisplayed
                    // }
                    w!("Error detected when parsing admin cmds");
                    state.0.codec.buffer(b"Command could not be executed\r\n");
                    state.0
                        .codec
                        .buffer(format!("Cause: {:?}\r\n", e.kind).as_bytes());
                    state.0.codec.buffer(
                        format!("Message:\r\n{}\r\n", e.message.replace("\n", "\r\n")).as_bytes(),
                    );
                    state.0
                        .codec
                        .buffer(format!("Aditional Info:\r\n{:?}\r\n", e.info).as_bytes());
                    d!("{:?}", e);
                    let _ = state.0.codec.poll_flush()?;
                    continue;
                }
                Ok(matches) => match args::AdminCmd::from_clap(&matches) {
                    args::AdminCmd::Peer(peercmd) => match peercmd {
                        args::PeerCmd::Add {
                            addr,
                            wait_handshake,
                        } => {
                            let mut state = state.take();
                            d!("Entered command: Adding a peer");
                            let wr = WorkerRequest::PeerAdd {
                                addr: addr,
                                wait_handshake: wait_handshake,
                                tx_sched: state.0.tx_sched.clone(),
                            };
                            let (peer, orx) = worker_request_wrapped!(state.0, wr, 200);
                            let next = SimpleWait(peer, orx);
                            transition!(next);
                        }
                        args::PeerCmd::Remove { actor_id } => {
                            let mut state = state.take();
                            d!("Entered command: Removing a peer");
                            let wr = WorkerRequest::PeerRemove { actor_id: actor_id };
                            let (peer, orx) = worker_request_wrapped!(state.0, wr, 200);
                            let next = SimpleWait(peer, orx);
                            transition!(next);
                        }
                        args::PeerCmd::List => {
                            let mut state = state.take();
                            d!("Entered command: Listing peers");
                            let wr = WorkerRequest::ListPeers {};
                            let (peer, orx) = worker_request_wrapped!(state.0, wr, 200);
                            let next = SimpleWait(peer, orx);
                            transition!(next);
                        }
                        _ => {}
                    },
                    args::AdminCmd::Wallet(_) => {}
                    args::AdminCmd::Blockchain(_) => {}
                    args::AdminCmd::Node(_) => {}
                    args::AdminCmd::Util(_) => {}
                    args::AdminCmd::Debug(debug) => match debug {
                        args::DebugCmd::Dummy => {
                            d!("started dummy cmd");
                            let wr = WorkerRequest::Hello;
                            let mut state = state.take();
                            let (mut peer, orx) = worker_request_wrapped!(state.0, wr, 200);
                            d!("Request sent to worker");
                            let next = SimpleWait(peer, orx);
                            transition!(next);
                        }
                        args::DebugCmd::Wait { delay } => {
                            i!("started wait cmd");
                            let wr = WorkerRequest::Wait { delay: delay };
                            let mut state = state.take();
                            let (mut peer, orx) = worker_request_wrapped!(state.0, wr, 200);
                            let next = SimpleWait(peer, orx);
                            transition!(next);
                        }
                        args::DebugCmd::PeerPrint => {
                            i!("started peerprint cmd");
                            let wr = WorkerRequest::PeerPrint;
                            let wrp = WorkerRequestPriority(wr, 200);
                            let (otx, orx) =
                                oneshot::channel::<Result<Box<WorkerResponseContent>>>();
                            let actor_id = state.0.actor_id;
                            let hello_index = 0;
                            let addr = AddrReqId(actor_id, hello_index);
                            let wrc = WorkerRequestContent(wrp, otx, addr);

                            let state = state.take();
                            state.0.tx_req.unbounded_send(Box::new(wrc)).expect(&ff!());

                            let next = SimpleWait(state.0, orx);
                            transition!(next);
                        }
                        args::DebugCmd::MsgFromHex { send, hex } => {
                            d!("started msgFromHex cmd");
                            let wr = WorkerRequest::MsgFromHex {
                                send: send,
                                binary: hex.0,
                            };

                            let mut state = state.take();
                            let (mut peer, orx) = worker_request_wrapped!(state.0, wr, 200);
                            d!("Request sent to worker");

                            let next = SimpleWait(peer, orx);
                            transition!(next);
                        }
                    },
                    args::AdminCmd::Exit => {
                        let mut state = state.take();
                        d!("Entered command: Exiting admin");
                        let wr = WorkerRequest::PeerRemove {
                            actor_id: state.0.actor_id,
                            //addr: state.0.codec.socket.peer_addr().expect(&ff!()),
                        };
                        let (mut peer, orx) = worker_request_wrapped!(state.0, wr, 200);
                        d!("Request sent to worker");
                        peer.push_ignored(orx);
                        let next = SelfRemove(peer);
                        transition!(next);
                    }
                },
            }

            // never reached
            match msg.as_ref() {
                "PING?" => {
                    d!("going to WAITING");
                    let state = state.take();
                    let next = Execution(state.0);
                    transition!(next)
                }
                _ => {
                    d!("BATATA: <{:?}>", &msg);
                }
            }
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
                                          //i!("{:#?}", &resp);

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

    fn poll_execution<'a>(
        state: &'a mut RentToOwn<'a, Execution>,
    ) -> Poll<AfterExecution, errors::Error> {
        while let Some(msg) = try_ready!(state.0.codec.poll()) {
            let msg = String::from_utf8(msg.to_vec()).expect(&ff!());

            match msg.as_ref() {
                "BYE" => {
                    state.0.codec.buffer(b"HAVE A GOOD ONE");
                    let _ = state.0.codec.poll_flush()?;

                    let state = state.take();
                    let next = End(state.0);
                    i!("going to END");
                    transition!(next)
                }
                _ => {}
            }
        }
        // Err(std::io::Error::new(std::io::ErrorKind::ConnectionAborted,
        // "Peer connection aborted."))
        panic!("Peer connection aborted.");
    }
}
