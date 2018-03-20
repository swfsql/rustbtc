mod composed_state;

use std;

use tokio::prelude::*;

use state_machine_future::RentToOwn;


use peer::Peer;

#[derive(StateMachineFuture)]
pub enum Machina {
    #[state_machine_future(start, transitions(Standby))]
    Welcome(Peer),

    #[state_machine_future(transitions(Waiting))]
    Standby(Peer),

    #[state_machine_future(transitions(ComposedState, End))]
    Waiting(Peer),

    #[state_machine_future(transitions(Standby, Waiting))]
    ComposedState(composed_state::MachinaFuture),

    #[state_machine_future(ready)]
    End(Peer),

    #[state_machine_future(error)]
    MachinaError(std::io::Error),
}

impl PollMachina for Machina {

    fn poll_welcome<'a>(
        peer: &'a mut RentToOwn<'a, Welcome>
    ) -> Poll<AfterWelcome, std::io::Error> {

        peer.0.lines.buffer("WELCOME".as_bytes());
        let _ = peer.0.lines.poll_flush()?;
        let _ = peer.0.lines.poll_flush()?; // to make sure
        println!("sent WELCOME");

        transition!(Standby(peer.take().0))
    }

    fn poll_standby<'a>(
        peer: &'a mut RentToOwn<'a, Standby>
    ) -> Poll<AfterStandby, std::io::Error> {

        while let Some(msg) = try_ready!(peer.0.lines.poll()) {
            let msg = String::from_utf8(msg.to_vec()).unwrap();

            match msg.as_ref() {
                "PING?" => {
                    println!("going to WAITING");
                    let peer = peer.take();
                    let waiting = Waiting(peer.0);
                    transition!(waiting)
                },
                _ => {
                    println!("BATATA: <{:?}>", &msg);
                },
            }
        }
        // Err(std::io::Error::new(std::io::ErrorKind::ConnectionAborted, "Peer connection aborted."))
        panic!("Peer connection aborted.");
    }

    fn poll_waiting<'a>(
        peer: &'a mut RentToOwn<'a, Waiting>
    ) -> Poll<AfterWaiting, std::io::Error> {

        while let Some(msg) = try_ready!(peer.0.lines.poll()) {
            let msg = String::from_utf8(msg.to_vec()).unwrap();

            match msg.as_ref() {
                "A" => {
                    peer.0.lines.buffer("Inside Composed State".as_bytes());
                    let _ = peer.0.lines.poll_flush()?;

                    let peer = peer.take();
                    let mach = composed_state::Machina::start(peer.0);
                    let next = ComposedState(mach);
                    println!("going to ComposedState");
                    transition!(next)
                },
                "BYE" => {
                    peer.0.lines.buffer("HAVE A GOOD ONE".as_bytes());
                    let _ = peer.0.lines.poll_flush()?;

                    let peer = peer.take();
                    let next = End(peer.0);
                    println!("going to END");
                    transition!(next)
                },
                _ => {

                },
            }
        }
        // Err(std::io::Error::new(std::io::ErrorKind::ConnectionAborted, "Peer connection aborted."))
        panic!("Peer connection aborted.");
    }

    fn poll_composed_state<'a>(
        mach: &'a mut RentToOwn<'a, ComposedState>
    ) -> Poll<AfterComposedState, std::io::Error> {

        let (mut peer, msg) = try_ready!(mach.0.poll());

        match msg.as_ref() {
            "PING" => {
                peer.lines.buffer("PONG".as_bytes());
                let _ = peer.lines.poll_flush()?;

                let next = Standby(peer);
                println!("going to Standby");
                transition!(next)
            },
            _ => {
                peer.lines.buffer("...".as_bytes());
                let _ = peer.lines.poll_flush()?;

                let next = Waiting(peer);
                println!("going to Waiting");
                transition!(next)
            },
        }
    }

}

