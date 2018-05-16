use actor::commons::channel_content::ActorId;
use actor::commons::{RxMpscRouterToPeer, RxOne, TxMpsc, TxMpscMainToSched};
use codec::msgs::msg::Msg;
use codec::msgs::Msgs;
use futures::Future;
use tokio::net::TcpStream;

pub mod machina;

pub struct Peer {
    codec: Msgs,
    rx_ignored: Vec<RxOne>,
    tx_req: TxMpsc,
    tx_sched: TxMpscMainToSched,
    rx_router: RxMpscRouterToPeer,
    actor_id: ActorId,
    request_counter: usize,
    version: Option<Msg>,
}

impl Peer {
    pub fn new(
        socket: TcpStream,
        tx_req: TxMpsc,
        tx_sched: TxMpscMainToSched,
        rx_router: RxMpscRouterToPeer,
        actor_id: ActorId,
    ) -> Peer {
        Peer {
            codec: Msgs::new(socket),
            rx_ignored: Vec::new(),
            tx_req: tx_req,
            tx_sched: tx_sched,
            rx_router: rx_router,
            actor_id,
            request_counter: 0,
            version: None,
        }
    } //

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
}
