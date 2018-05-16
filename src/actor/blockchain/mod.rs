/*use codec::msgs::Msgs;
use actor::commons::{RxOne,TxMpsc, TxMpscMainToSched, RxMpscWorkerToBlockChain};
use futures::sync::mpsc;
use futures::Future;
use std::sync::{Arc, Mutex};
use tokio::net::TcpStream;
*/
use actor::commons::{RxMpscWorkerToBlockChain, TxMpsc};
pub mod machina;

pub struct Blockchain {
    //TODO Blockchain && blockheaders
    tx_bchain_to_sched: TxMpsc,
    rx_worker_to_bchain: RxMpscWorkerToBlockChain,
}

impl Blockchain {
    pub fn new(
        tx_bchain_to_sched: TxMpsc,
        rx_worker_to_bchain: RxMpscWorkerToBlockChain,
    ) -> Blockchain {
        Blockchain {
            tx_bchain_to_sched,
            rx_worker_to_bchain,
        }
    }
    /*
    pub fn push_ignored(&mut self, rx: RxOne) {
        self.rx_ignored.push(rx);
    }

    pub fn poll_ignored(&mut self) {
        let removed_indices = self.rx_ignored
            .iter_mut()
            .enumerate()
            .map(|(i, rx)| (i, rx.poll()))
            .filter(|&(_i, ref fut)| {
                fut.is_err() || (fut.is_ok() && fut.as_ref().expect(&ff!()).is_ready())
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
    */
}
