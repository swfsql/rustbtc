extern crate tokio;

use std;
use tokio::prelude::*;
use state_machine_future::RentToOwn;

use peer::Peer;

#[derive(StateMachineFuture)]
pub enum Machina {
    #[state_machine_future(start, transitions(B, End))]
    A(Peer),

    #[state_machine_future(transitions(End))]
    B(Peer),

    #[state_machine_future(ready)]
    End((Peer, String)),

    #[state_machine_future(error)]
    Error(std::io::Error),
}

impl PollMachina for Machina {
    fn poll_a<'a>(peer: &'a mut RentToOwn<'a, A>) -> Poll<AfterA, std::io::Error> {
        while let Some(msg) = try_ready!(peer.0.lines.poll()) {
            let msg = String::from_utf8(msg.to_vec()).unwrap();

            match msg.as_ref() {
                "B" => {
                    peer.0.lines.buffer(b"GOING TO B");
                    let _ = peer.0.lines.poll_flush()?;

                    let next = B(peer.take().0);
                    i!("going to B state");
                    transition!(next)
                }
                _ => {
                    peer.0.lines.buffer(b"...");
                    let _ = peer.0.lines.poll_flush()?;

                    let next = End((peer.take().0, msg));
                    i!("going to End state");
                    transition!(next)
                }
            }
        }
        // Err(std::io::Error::new(std::io::ErrorKind::ConnectionAborted,
        //  "Peer connection aborted."))
        panic!("Peer connection aborted.");
    }

    fn poll_b<'a>(peer: &'a mut RentToOwn<'a, B>) -> Poll<AfterB, std::io::Error> {
        while let Some(msg) = try_ready!(peer.0.lines.poll()) {
            let msg = String::from_utf8(msg.to_vec()).unwrap();

            let peer = peer.take();
            let next = End((peer.0, msg));
            i!("going to End state");
            transition!(next)
        }
        // Err(std::io::Error::new(std::io::ErrorKind::ConnectionAborted, "Peer connection aborted."))
        panic!("Peer connection aborted.");
    }
}
