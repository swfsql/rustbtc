use std;

use tokio::prelude::*;

use state_machine_future::RentToOwn;

use admin::Peer;
use admin::args;

use structopt::StructOpt;

//use exec::commons::{AddrReqId, RequestId, RxMpscSf, RxOne, TxMpsc,
//                   TxOne, WorkerRequest, WorkerRequestContent, WorkerRequestPriority,
//                  WorkerResponseContent, RxPeers};

use exec::commons::{AddrReqId, RxOne,
                     WorkerRequest, WorkerRequestContent, WorkerRequestPriority,
                    WorkerResponseContent, WorkerToPeerRequestAndPriority, PeerRequest,MainToSchedRequestContent};


//use futures::sync::{mpsc, oneshot};
use futures::sync::{oneshot};
//use futures;
//use std::io::{Error, ErrorKind};

use env_logger::LogBuilder;
#[macro_use]
use macros;

use codec;
use hex::ToHex;

//use macros;
/*
use macros::*;
*/

#[derive(StateMachineFuture)]
pub enum Machina {
    #[state_machine_future(start, transitions(Standby))]
    Welcome(Peer),

    #[state_machine_future(transitions(Execution,SimpleWait,SelfRemove,End))]
    Standby(Peer),

    #[state_machine_future(transitions(Standby))]
    SimpleWait(Peer,RxOne),

    #[state_machine_future(transitions(End))]
    SelfRemove(Peer),

    #[state_machine_future(transitions(End))]
    Execution(Peer),

    #[state_machine_future(ready)]
    End(Peer),

    #[state_machine_future(error)]
    MachinaError(std::io::Error),
}

impl PollMachina for Machina {
    fn poll_welcome<'a>(
        peer: &'a mut RentToOwn<'a, Welcome>,
    ) -> Poll<AfterWelcome, std::io::Error> {
        peer.0.lines.buffer(b"WELCOME\r\n");
        let _ = peer.0.lines.poll_flush()?;
        let _ = peer.0.lines.poll_flush()?; // to make sure
        d!("sent WELCOME");

        transition!(Standby(peer.take().0))
    }

    fn poll_standby<'a>(
        peer: &'a mut RentToOwn<'a, Standby>,
    ) -> Poll<AfterStandby, std::io::Error> {

        d!("poll standby");

        defmac!(prepare_transition mut state_peer, wr, priority => {
            let wrp = WorkerRequestPriority(wr, priority);
            let (otx, orx) = oneshot::channel::<Result<Box<WorkerResponseContent>, _>>();
            let skt = state_peer.lines.socket.peer_addr().unwrap();
            let addr = AddrReqId(skt, state_peer.next_request_counter());
            let wrc = WorkerRequestContent(wrp, otx, addr);
            state_peer.tx_req.unbounded_send(Box::new(wrc)).unwrap();
            (state_peer, orx)
        });

        peer.0.poll_ignored();

        loop {
            if let Ok(Async::Ready(Some(box WorkerToPeerRequestAndPriority(peer_req, priority)))) = peer.0.rx_toolbox.poll() {
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

        while let Some(msg) = try_ready!(peer.0.lines.poll()) {
            let msg = String::from_utf8(msg.to_vec()).unwrap();

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
                    peer.0.lines.buffer(b"Command could not be executed\r\n");
                    peer.0
                        .lines
                        .buffer(format!("Cause: {:?}\r\n", e.kind).as_bytes());
                    peer.0.lines.buffer(
                        format!("Message:\r\n{}\r\n", e.message.replace("\n", "\r\n")).as_bytes(),
                    );
                    peer.0
                        .lines
                        .buffer(format!("Aditional Info:\r\n{:?}\r\n", e.info).as_bytes());
                    d!("{:?}", e);
                    let _ = peer.0.lines.poll_flush()?;
                    continue;
                }
                Ok(matches) => match args::AdminCmd::from_clap(&matches) {
                    args::AdminCmd::Peer(peercmd) => match peercmd {
                        args::PeerCmd::Add{addr, wait_handhsake} => {

                            let peer = peer.take();
                            d!("Entered command: Adding a peer");
                            let wr = WorkerRequest::PeerAdd{addr: addr, wait_handhsake:wait_handhsake, tx_sched: peer.0.tx_sched.clone()};
                            let (peer, orx) = prepare_transition!(peer.0, wr, 200);
                            let next = SimpleWait(peer,orx);
                            transition!(next);
                        },
                        args::PeerCmd::Remove{addr} => {

                            let state = peer.take();
                            d!("Entered command: Removing a peer");
                            let wr = WorkerRequest::PeerRemove{addr: addr};
                            let (peer, orx) = prepare_transition!(state.0, wr, 200);
                            let next = SimpleWait(peer,orx);
                            transition!(next);

                        },
                        args::PeerCmd::List => {
                            let state = peer.take();
                            d!("Entered command: Removing a peer");
                            let wr = WorkerRequest::ListPeers{};
                            let (peer, orx) = prepare_transition!(state.0, wr, 200);
                            let next = SimpleWait(peer,orx);
                            transition!(next);
                        },
                        _ => {},
                    }
                    args::AdminCmd::Wallet(_) => {}
                    args::AdminCmd::Blockchain(_) => {}
                    args::AdminCmd::Node(_) => {}
                    args::AdminCmd::Util(_) => {}
                    args::AdminCmd::Debug(debug) => match debug {
                        args::DebugCmd::Dummy => {
                            d!("started dummy cmd");
                            let wr = WorkerRequest::Hello;


                            let state = peer.take();
                            let (mut peer, orx) = prepare_transition!(state.0, wr, 200);
                            d!("Request sent to worker");

                            /*
                            let wrp = WorkerRequestPriority(wr, 200);
                            let (otx, orx) = oneshot::channel::<Result<Box<WorkerResponseContent>, _>>();
                            let skt = peer.0.lines.socket.peer_addr().unwrap();
                            let hello_index = 0;
                            let addr = AddrReqId(skt, hello_index);
                            let wrc = WorkerRequestContent(wrp, otx, addr);

                            let peer = peer.take();
                            peer.0.tx_req.unbounded_send(Box::new(wrc)).unwrap();
                            */

                            let next = SimpleWait(peer,orx);
                            transition!(next);

                        },
                        args::DebugCmd::Wait{delay} => {
                            i!("started wait cmd");
                            let wr = WorkerRequest::Wait{delay: delay};
                            let wrp = WorkerRequestPriority(wr, 200);
                            let (otx, orx) = oneshot::channel::<Result<Box<WorkerResponseContent>, _>>();
                            let skt = peer.0.lines.socket.peer_addr().unwrap();
                            let hello_index = 0;
                            let addr = AddrReqId(skt, hello_index);
                            let wrc = WorkerRequestContent(wrp, otx, addr);

                            let peer = peer.take();
                            peer.0.tx_req.unbounded_send(Box::new(wrc)).unwrap();

                            let next = SimpleWait(peer.0,orx);
                            transition!(next);
                        },
                        args::DebugCmd::PeerPrint => {
                            i!("started peerprint cmd");
                            let wr = WorkerRequest::PeerPrint;
                            let wrp = WorkerRequestPriority(wr, 200);
                            let (otx, orx) = oneshot::channel::<Result<Box<WorkerResponseContent>, _>>();
                            let skt = peer.0.lines.socket.peer_addr().unwrap();
                            let hello_index = 0;
                            let addr = AddrReqId(skt, hello_index);
                            let wrc = WorkerRequestContent(wrp, otx, addr);

                            let peer = peer.take();
                            peer.0.tx_req.unbounded_send(Box::new(wrc)).unwrap();

                            let next = SimpleWait(peer.0,orx);
                            transition!(next);

                        },
                        args::DebugCmd::MsgFromHex{send, hex} => {
                            d!("started msgFromHex cmd");
                            let wr = WorkerRequest::MsgFromHex{send:send, binary: hex.0};

                            let state = peer.take();
                            let (mut peer, orx) = prepare_transition!(state.0, wr, 200);
                            d!("Request sent to worker");

                            /*
                            let wrp = WorkerRequestPriority(wr, 200);
                            let (otx, orx) = oneshot::channel::<Result<Box<WorkerResponseContent>, _>>();
                            let skt = peer.0.lines.socket.peer_addr().unwrap();
                            let hello_index = 0;
                            let addr = AddrReqId(skt, hello_index);
                            let wrc = WorkerRequestContent(wrp, otx, addr);

                            let peer = peer.take();
                            peer.0.tx_req.unbounded_send(Box::new(wrc)).unwrap();
                            */

                            let next = SimpleWait(peer,orx);
                            transition!(next);
                        },
                    },
                    args::AdminCmd::Exit => {
                        let state = peer.take();
                        d!("Entered command: Exiting admin");
                        let wr = WorkerRequest::PeerRemove{addr: state.0.lines.socket.peer_addr().unwrap()};
                        let (mut peer, orx) = prepare_transition!(state.0, wr, 200);
                        d!("Request sent to worker");
                        peer.push_ignored(orx);
                        let next = SelfRemove(peer);
                        transition!(next);
                    },
                },
            }

            // never reached
            match msg.as_ref() {
                "PING?" => {
                    d!("going to WAITING");
                    let peer = peer.take();
                    let next = Execution(peer.0);
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
    ) -> Poll<AfterSimpleWait, std::io::Error> {
        d!("SimpleWait pool called");

        let resp;
        match state.1.poll() {
            Ok(Async::Ready(fresp)) => {
                resp = fresp;
            },
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

        peer
            .lines
            .buffer(format!("{:#?}", &resp).as_bytes());
        let _ = peer.lines.poll_flush()?;
        let _ = peer.lines.poll_flush()?; // to make sure
        //i!("{:#?}", &resp);

        //orx.take();
        let next = Standby(peer);
        transition!(next)
    }

    fn poll_self_remove<'a>(
        state: &'a mut RentToOwn<'a, SelfRemove>,
    ) -> Poll<AfterSelfRemove, std::io::Error> {

        d!("Entered self_remove state!");
        state.0.poll_ignored();
        if state.0.rx_ignored.len() > 0 {
            return Ok(Async::NotReady);
        }
        d!("Dumped all trash responses!");
        let state = state.take();
        let addr = state.0.lines.socket.peer_addr().unwrap();
        let msg = MainToSchedRequestContent::Unregister(addr);
        state.0.tx_sched.lock().unwrap().unbounded_send(Box::new(msg));
        let next = End(state.0); //Calling this simplewait???
        transition!(next);
    }

    fn poll_execution<'a>(
        peer: &'a mut RentToOwn<'a, Execution>,
    ) -> Poll<AfterExecution, std::io::Error> {
        while let Some(msg) = try_ready!(peer.0.lines.poll()) {
            let msg = String::from_utf8(msg.to_vec()).unwrap();

            match msg.as_ref() {
                "BYE" => {
                    peer.0.lines.buffer(b"HAVE A GOOD ONE");
                    let _ = peer.0.lines.poll_flush()?;

                    let peer = peer.take();
                    let next = End(peer.0);
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
