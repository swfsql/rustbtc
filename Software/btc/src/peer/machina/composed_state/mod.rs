extern crate tokio;

use std;
use tokio::prelude::*;
use state_machine_future::RentToOwn;

use peer::Peer;

#[derive(StateMachineFuture)]
pub enum Machina {
    #[state_machine_future(start, transitions(InnerB,InnerEnd))]
    InnerA(Peer),

    #[state_machine_future(transitions(InnerEnd))]
    InnerB(Peer),

    #[state_machine_future(ready)]
    InnerEnd((Peer, String)),

    #[state_machine_future(error)]
    InnerError(std::io::Error),
}


impl PollMachina for Machina {
    fn poll_inner_a<'a>(
        peer: &'a mut RentToOwn<'a, InnerA>
    ) -> Poll<AfterInnerA, std::io::Error> {

        while let Some(msg) = try_ready!(peer.0.lines.poll()) {
            let msg = String::from_utf8(msg.to_vec()).unwrap();

            match msg.as_ref() {
                "B" => {
                    peer.0.lines.buffer("GOING TO B".as_bytes());
                    let _ = peer.0.lines.poll_flush()?;

                    let next = InnerB(peer.take().0);
                    println!("going to InnerB");
                    transition!(next)
                },
                _ =>  {
                    peer.0.lines.buffer("...".as_bytes());
                    let _ = peer.0.lines.poll_flush()?;

                    let next = InnerEnd((peer.take().0, msg));
                    println!("going to InnerEnd");
                    transition!(next)
                },
            }
        }
        // Err(std::io::Error::new(std::io::ErrorKind::ConnectionAborted, "Peer connection aborted."))
        panic!("Peer connection aborted.");
    }

    fn poll_inner_b<'a>(
        peer: &'a mut RentToOwn<'a, InnerB>
    ) -> Poll<AfterInnerB, std::io::Error> {

        while let Some(msg) = try_ready!(peer.0.lines.poll()) {
            let msg = String::from_utf8(msg.to_vec()).unwrap();

            let peer = peer.take();
            let next = InnerEnd((peer.0, msg));
            println!("going to InnerEnd");
            transition!(next)
        }
        // Err(std::io::Error::new(std::io::ErrorKind::ConnectionAborted, "Peer connection aborted."))
        panic!("Peer connection aborted.");
    }
}

