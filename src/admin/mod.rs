use codec::lines::Lines;
use futures::sync::mpsc;
use futures::Future;
use std::sync::{Arc, Mutex};
use tokio::net::TcpStream;

use exec::commons::{RxOne, TxMpscMainToSched, WorkerRequestContent, WorkerToPeerRequestAndPriority};

pub mod args;
pub mod machina;
//use::macros;

pub struct Peer {
    codec: Lines,
    rx_ignored: Vec<RxOne>,
    tx_req: mpsc::UnboundedSender<Box<WorkerRequestContent>>,
    tx_sched: Arc<Mutex<TxMpscMainToSched>>,
    _rx_toolbox: mpsc::UnboundedReceiver<Box<WorkerToPeerRequestAndPriority>>,
    request_counter: usize,
}

impl Peer {
    pub fn new(
        socket: TcpStream,
        tx_req: mpsc::UnboundedSender<Box<WorkerRequestContent>>,
        tx_sched: Arc<Mutex<TxMpscMainToSched>>,
        rx_toolbox: mpsc::UnboundedReceiver<Box<WorkerToPeerRequestAndPriority>>,
    ) -> Peer {
        Peer {
            codec: Lines::new(socket),
            rx_ignored: Vec::new(),
            tx_req: tx_req,
            tx_sched: tx_sched,
            _rx_toolbox: rx_toolbox,
            request_counter: 0,
        }
    }

    pub fn push_ignored(&mut self, rx: RxOne) {
        self.rx_ignored.push(rx);
    }

    pub fn poll_ignored(&mut self) {
        let removed_indices = self.rx_ignored
            .iter_mut()
            .enumerate()
            .map(|(i, rx)| (i, rx.poll()))
            .filter(|&(_i, ref fut)| {
                fut.is_err() || (fut.is_ok() && fut.as_ref().unwrap().is_ready())
            })
            .inspect(|&(_i, ref rx)| i!("Oneshot response arrived, and got ignored: \n{:#?}", rx))
            .map(|(i, _rx)| i)
            .collect::<Vec<_>>();
        for i in removed_indices.iter().rev() {
            let _ = self.rx_ignored.swap_remove(*i);
        }
    }

    pub fn next_request_counter(&mut self) -> usize {
        self.request_counter += 1;
        self.request_counter
    }
}
