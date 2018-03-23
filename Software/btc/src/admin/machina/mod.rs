use std;

use tokio::prelude::*;

use state_machine_future::RentToOwn;

use admin::Peer;
use admin::args;

use structopt::StructOpt;

#[derive(StateMachineFuture)]
pub enum Machina {
    #[state_machine_future(start, transitions(Standby))]
    Welcome(Peer),

    #[state_machine_future(transitions(Execution))]
    Standby(Peer),

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
                    peer.0.lines.buffer(format!("Cause: {:?}\r\n", e.kind).as_bytes());
                    peer.0.lines.buffer(format!(
                        "Message:\r\n{}\r\n", e.message.replace("\n", "\r\n")).as_bytes()
                    );
                    peer.0.lines.buffer(format!("Aditional Info:\r\n{:?}\r\n", e.info).as_bytes());
                    println!("{:?}", e);
                    let _ = peer.0.lines.poll_flush()?;
                    continue;
                },
                Ok(matches) => match args::AdminCmd::from_clap(&matches) {
                    args::AdminCmd::Peer(_) => {},
                    args::AdminCmd::Wallet(_) => {},
                    args::AdminCmd::Blockchain(_) => {},
                    args::AdminCmd::Node(_) => {},
                    args::AdminCmd::Util(_) => {},
                    args::AdminCmd::Debug(_) => {},
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
