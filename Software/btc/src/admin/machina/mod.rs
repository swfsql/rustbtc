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
                    WorkerResponseContent, WorkerToPeerRequestAndPriority, PeerRequest};


//use futures::sync::{mpsc, oneshot};
use futures::sync::{oneshot};
//use futures;
//use std::io::{Error, ErrorKind};

use env_logger::LogBuilder;
#[macro_use]
use macros;

//use macros;
/*
use macros::*;
*/

#[derive(StateMachineFuture)]
pub enum Machina {
    #[state_machine_future(start, transitions(Standby))]
    Welcome(Peer),

    #[state_machine_future(transitions(Execution,WaitHello,WaitPeerAdd,WaitPeerPrint))]
    Standby(Peer),

    #[state_machine_future(transitions(Standby))]
    WaitPeerPrint(Peer,RxOne),

    #[state_machine_future(transitions(Standby))]
    WaitHello(Peer,RxOne),

    #[state_machine_future(transitions(Standby))]
    WaitPeerAdd(Peer,RxOne),

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
        println!("admin:: sent WELCOME");

        transition!(Standby(peer.take().0))
    }

    fn poll_standby<'a>(
        peer: &'a mut RentToOwn<'a, Standby>,
    ) -> Poll<AfterStandby, std::io::Error> {

        e!("admin got polled!!");

        loop {
            println!("test");
            if let Ok(Async::Ready(Some(box WorkerToPeerRequestAndPriority(peer_req, priority)))) = peer.0.rx_toolbox.poll() {
                match peer_req {

                    PeerRequest::Dummy => {
                        println!("admin:: received dummy command, read on standby");
                    },
                    _ => {println!("admin:: loop de recibo inner");
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
                    println!("admin:: Error detected when parsing admin cmds");
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
                    println!("admin:: {:?}", e);
                    let _ = peer.0.lines.poll_flush()?;
                    continue;
                }
                Ok(matches) => match args::AdminCmd::from_clap(&matches) {
                    args::AdminCmd::Peer(peercmd) => match peercmd {
                        args::PeerCmd::Add{addr, wait_handhsake} => {

                            let peer = peer.take();

                            println!("admin:: started dummy cmd");
                            let wr = WorkerRequest::PeerAdd{addr: addr, wait_handhsake:wait_handhsake, tx_sched: peer.0.tx_sched.clone()};
                            let wrp = WorkerRequestPriority(wr, 200);
                            let (otx, orx) = oneshot::channel::<Result<Box<WorkerResponseContent>, _>>();
                            let skt = peer.0.lines.socket.peer_addr().unwrap();
                            let hello_index = 0;
                            let addr = AddrReqId(skt, hello_index);
                            let wrc = WorkerRequestContent(wrp, otx, addr);

                            peer.0.tx_req.unbounded_send(Box::new(wrc)).unwrap();

                            let next = WaitHello(peer.0,orx);
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
                            println!("admin:: started dummy cmd");
                            let wr = WorkerRequest::Hello;
                            let wrp = WorkerRequestPriority(wr, 200);
                            let (otx, orx) = oneshot::channel::<Result<Box<WorkerResponseContent>, _>>();
                            let skt = peer.0.lines.socket.peer_addr().unwrap();
                            let hello_index = 0;
                            let addr = AddrReqId(skt, hello_index);
                            let wrc = WorkerRequestContent(wrp, otx, addr);

                            let peer = peer.take();
                            peer.0.tx_req.unbounded_send(Box::new(wrc)).unwrap();

                            let next = WaitHello(peer.0,orx);
                            transition!(next);

                        },
                        args::DebugCmd::Wait{delay} => {
                            println!("admin:: started wait cmd");
                            let wr = WorkerRequest::Wait{delay: delay};
                            let wrp = WorkerRequestPriority(wr, 200);
                            let (otx, orx) = oneshot::channel::<Result<Box<WorkerResponseContent>, _>>();
                            let skt = peer.0.lines.socket.peer_addr().unwrap();
                            let hello_index = 0;
                            let addr = AddrReqId(skt, hello_index);
                            let wrc = WorkerRequestContent(wrp, otx, addr);

                            let peer = peer.take();
                            peer.0.tx_req.unbounded_send(Box::new(wrc)).unwrap();

                            let next = WaitHello(peer.0,orx);
                            transition!(next);
                        },
                        args::DebugCmd::PeerPrint => {
                            println!("admin:: started peerprint cmd");
                            let wr = WorkerRequest::PeerPrint;
                            let wrp = WorkerRequestPriority(wr, 200);
                            let (otx, orx) = oneshot::channel::<Result<Box<WorkerResponseContent>, _>>();
                            let skt = peer.0.lines.socket.peer_addr().unwrap();
                            let hello_index = 0;
                            let addr = AddrReqId(skt, hello_index);
                            let wrc = WorkerRequestContent(wrp, otx, addr);

                            let peer = peer.take();
                            peer.0.tx_req.unbounded_send(Box::new(wrc)).unwrap();

                            let next = WaitPeerPrint(peer.0,orx);
                            transition!(next);

                        },
                    },
                },
            }

            // never reached
            match msg.as_ref() {
                "PING?" => {
                    println!("admin:: going to WAITING");
                    let peer = peer.take();
                    let next = Execution(peer.0);
                    transition!(next)
                }
                _ => {
                    println!("admin:: BATATA: <{:?}>", &msg);
                }
            }
        }
        // Err(std::io::Error::new(std::io::ErrorKind::ConnectionAborted, "Peer connection aborted."))
        panic!("Peer connection aborted.");
    }

    fn poll_wait_hello<'a>(
        wait_hello: &'a mut RentToOwn<'a, WaitHello>,
    ) -> Poll<AfterWaitHello, std::io::Error> {
        println!("admin:: WaitHello poll");

        let resp;
        match wait_hello.1.poll() {
            Ok(Async::Ready(fresp)) => {
                resp = fresp;
                println!("admin:: 111111111111 admin WaitHello poll");
            },
            Ok(Async::NotReady) => {
                return Ok(Async::NotReady);
            }
            Err(_) => panic!("Canceled scheduler response"),
        };

        let wait_hello = wait_hello.take();
        let mut peer = wait_hello.0;
        let _orx = wait_hello.1;

        peer
            .lines
            .buffer(format!("{:#?}", &resp).as_bytes());
        let _ = peer.lines.poll_flush()?;
        let _ = peer.lines.poll_flush()?; // to make sure
        println!("admin:: {:#?}", &resp);

        //orx.take();
        let next = Standby(peer);
        transition!(next)
    }

    fn poll_wait_peer_print<'a>(
        state: &'a mut RentToOwn<'a, WaitPeerPrint>,
    ) -> Poll<AfterWaitPeerPrint, std::io::Error> {
        println!("admin:: WaitPeerPrint poll");

        let resp;
        match state.1.poll() {
            Ok(Async::Ready(fresp)) => {
                println!("admin:: received response from worker (PeerPrint)");
                resp = fresp;
            },
            Ok(Async::NotReady) => {
                return Ok(Async::NotReady);
            }
            Err(_) => panic!("Canceled scheduler response"),
        };

        let state = state.take();
        let mut peer = state.0;
        let _orx = state.1;

        peer
            .lines
            .buffer(format!("{:#?}", &resp).as_bytes());
        let _ = peer.lines.poll_flush()?;
        let _ = peer.lines.poll_flush()?; // to make sure
        println!("admin:: {:#?}", &resp);

        //orx.take();
        let next = Standby(peer);
        transition!(next)
    }

    fn poll_wait_peer_add<'a>(
        wait_peer_add: &'a mut RentToOwn<'a, WaitPeerAdd>,
    ) -> Poll<AfterWaitPeerAdd, std::io::Error> {
        println!("admin:: WaitPeerAdd poll");

        let resp;
        match wait_peer_add.1.poll() {
            Ok(Async::Ready(fresp)) => {
                resp = fresp;
                println!("admin:: 111111111111 admin WaitHello poll");
            },
            Ok(Async::NotReady) => {
                return Ok(Async::NotReady);
            }
            Err(_) => panic!("Canceled scheduler response"),
        };

        let wait_peer_add = wait_peer_add.take();
        let mut peer = wait_peer_add.0;
        let _orx = wait_peer_add.1;

        peer
            .lines
            .buffer(format!("{:#?}", &resp).as_bytes());
        let _ = peer.lines.poll_flush()?;
        let _ = peer.lines.poll_flush()?; // to make sure
        println!("admin:: {:#?}", &resp);

        //orx.take();
        let next = Standby(peer);
        transition!(next)
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
                    println!("admin:: going to END");
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
