use std;

use tokio::prelude::*;

use state_machine_future::RentToOwn;

use admin::Peer;
use admin::args;

use structopt::StructOpt;

use scheduler::commons::{AddrReqId, RequestId, Rx_mpsc_sf, Rx_one, Tx_mpsc,
                    Tx_one, WorkerRequest, WorkerRequestContent, WorkerRequestPriority,
                    WorkerResponseContent, Rx_peers};
use futures::sync::{mpsc, oneshot};
use futures;
use std::io::{Error, ErrorKind};

#[derive(StateMachineFuture)]
pub enum Machina {
    #[state_machine_future(start, transitions(Standby))]
    Welcome(Peer),

    #[state_machine_future(transitions(Execution,WaitHello))]
    Standby(Peer),

    #[state_machine_future(transitions(Standby))]
    WaitHello(Peer,Rx_one),

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
        println!("sent WELCOME");

        transition!(Standby(peer.take().0))
    }

    fn poll_standby<'a>(
        peer: &'a mut RentToOwn<'a, Standby>,
    ) -> Poll<AfterStandby, std::io::Error> {
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
                    println!("Error detected when parsing admin cmds");
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
                    println!("{:?}", e);
                    let _ = peer.0.lines.poll_flush()?;
                    continue;
                }
                Ok(matches) => match args::AdminCmd::from_clap(&matches) {
                    args::AdminCmd::Peer(_) => {}
                    args::AdminCmd::Wallet(_) => {}
                    args::AdminCmd::Blockchain(_) => {}
                    args::AdminCmd::Node(_) => {}
                    args::AdminCmd::Util(_) => {}
                    args::AdminCmd::Debug(debug) => match debug {
                        Dummy => {
                            let wr = WorkerRequest::Hello;
                            let wrp = WorkerRequestPriority(wr, 200);
                            let (otx, orx) = oneshot::channel::<Result<Box<WorkerResponseContent>, _>>();
                            let skt = peer.0.lines.socket.peer_addr().unwrap();
                            let hello_index = 0;
                            let addr = AddrReqId(skt, hello_index);
                            let wrc = WorkerRequestContent(wrp, otx, addr);

                            let peer = peer.take();
                            peer.0.tx_req.unbounded_send(Box::new(wrc));

                            let next = WaitHello(peer.0,orx);
                            return transition!(next);

/*
pub struct AddrReqId(pub SocketAddr, pub RequestId);

pub struct WorkerRequestContent(
    pub WorkerRequestPriority,
    pub Tx_one,
    pub AddrReqId);

pub struct WorkerRequestPriority(
    pub WorkerRequest,
    pub RequestPriority);


pub enum WorkerRequest {
    NewPeer { addr: SocketAddr },
    KillPeer { addr: SocketAddr },
    InfoPeer { addr: SocketAddr },
    ListPeers,
    SendPing { addr: SocketAddr },
    Hello,
}
*/


                        },
                    },
                },
            }

            // never reached
            match msg.as_ref() {
                "PING?" => {
                    println!("going to WAITING");
                    let peer = peer.take();
                    let next = Execution(peer.0);
                    transition!(next)
                }
                _ => {
                    println!("BATATA: <{:?}>", &msg);
                }
            }
        }
        // Err(std::io::Error::new(std::io::ErrorKind::ConnectionAborted, "Peer connection aborted."))
        panic!("Peer connection aborted.");
    }

    fn poll_wait_hello<'a>(
        wait_hello: &'a mut RentToOwn<'a, WaitHello>,
    ) -> Poll<AfterWaitHello, std::io::Error> {

        let resp;
        match wait_hello.1.poll() {
            Ok(Async::Ready(fresp)) => {
                resp = fresp;
            },
            Ok(Async::NotReady) => {
                return Ok(Async::NotReady);
            }
            Err(_) => panic!("Canceled scheduler response"),
        };

        let wait_hello = wait_hello.take();
        let mut peer = wait_hello.0;
        let mut orx = wait_hello.1;

        peer
            .lines
            .buffer(format!("{:#?}", &resp).as_bytes());
        println!("{:#?}", &resp);

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
                    println!("going to END");
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
