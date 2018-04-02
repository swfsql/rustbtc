#![recursion_limit = "1024"]
#[macro_use]
extern crate error_chain;
mod errors {
    error_chain!{}
}
use errors::*;

extern crate state_machine_future;

extern crate env_logger;
#[macro_use]
extern crate log;

extern crate hex;
extern crate time;

#[macro_use]
extern crate btc;

//use futures::sync::{mpsc, oneshot};
use futures::sync::{mpsc};
use btc::exec::commons;
use structopt::StructOpt;


// use btc::commons::new_from_hex::NewFromHex;
// use btc::commons::into_bytes::IntoBytes;

// usually ran with:
// RUST_LOG=btc=INFO cargo run

extern crate tokio;

extern crate bytes;
extern crate futures;

#[macro_use]
extern crate structopt;

use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;

use std::net::SocketAddr;
use std::thread;
use std::sync::{Arc, Mutex};


#[derive(StructOpt, Debug)]
#[structopt(name = "")]
/// Node general settings
pub struct EnvVar {
  #[structopt(long = "node-socket-addr", default_value = "127.0.0.1:8080")]
  node_addr: SocketAddr,

  #[structopt(long = "admin-socket-addr", default_value = "127.0.0.1:8081")]
  admin_addr: SocketAddr,
}

fn process_peer(socket: TcpStream, _tx_sched: Arc<Mutex<commons::TxMpscMainToSched>>) {
    let peer = btc::peer::Peer::new(socket);

    //        .map_err(|_| ());

    let peer_machina = btc::peer::machina::Machina::start(peer)
        .map_err(|_| ())
        .map(|_| ());

    tokio::spawn(peer_machina);
    println!("depois do spawn");
}
fn process_admin(socket: TcpStream, tx_sched: Arc<Mutex<commons::TxMpscMainToSched>>) {
    e!("ahsuhsua");
    let (tx_peer, rx_peer) = mpsc::unbounded();
    let (tx_toolbox, rx_toolbox) = mpsc::unbounded();
    {
        let tx_sched_unlocked = tx_sched.lock().unwrap();
        tx_sched_unlocked.unbounded_send(
            Box::new(commons::MainToSchedRequestContent(
            commons::RxPeers(socket.peer_addr().unwrap(), rx_peer.into_future()),
            tx_toolbox,
            ))
        ).unwrap();
    }

    let peer = btc::admin::Peer::new(socket, tx_peer, tx_sched, rx_toolbox);

    //        .map_err(|_| ());

    let peer_machina = btc::admin::machina::Machina::start(peer)
        .map_err(|_| ())
        .map(|_| ());

    tokio::spawn(peer_machina);
    println!("depois do spawn");
}

use env_logger::LogBuilder;

fn run() -> Result<()> {


    LogBuilder::new()
        .format(|record| {
                    format!("[{}]{}",
                            record.level(),
                            record.args())
                })
        .parse(&std::env::var("RUST_LOG").unwrap_or_default())
        .init().unwrap();

    //env_logger::init().unwrap();
    let args = EnvVar::from_args();

    e!("Testing print aheuheau eahueuaheua uheaheuau");
    w!("Testing printhuhuhu huhuhu huhuhu");
    i!("Testing print llalal lalal");
    d!("Testing print xxx");


    let (tx, rx) = mpsc::unbounded();
    let mtx = Arc::new(Mutex::new(tx));
    let scheduler = btc::exec::scheduler::Scheduler::new(rx, 3)
        .map_err(|_| ());
    thread::spawn(move || {
        tokio::run(scheduler);
    });

    //let addr_peer = "127.0.0.1:8080".parse().unwrap();
    //let addr_admin = "127.0.0.1:8081".parse().unwrap();

    let listener_peer = TcpListener::bind(&args.node_addr).unwrap();
    let listener_admin = TcpListener::bind(&args.admin_addr).unwrap();

    let server_listeners = listener_admin
        .incoming()
        .map(|socket| (socket, true))
        .select(listener_peer
            .incoming()
            .map(|socket| (socket, false)))
        .for_each(move |(socket, is_admin)| {
            if is_admin {
                process_admin(socket, Arc::clone(&mtx));
            } else {
                process_peer(socket, Arc::clone(&mtx));
            }
            Ok(())
        })
        .map_err(|err| {
            println!("accept error = {:?}", err);
        });



    println!("server_peer running on {:#?}", args.node_addr);
    println!("server_admin running on {:#?}", args.admin_addr);

    /*
    thread::spawn(move || {
        tokio::run(server_admin);
    });
    */

    //tokio::run(server_peer);
    tokio::run(server_listeners);

    info!(
        "\n\
         ---------------------end-\n\
         {}",
        time::now().strftime("%Hh%Mm%Ss - D%d/M%m/Y%Y").unwrap()
    );
    Ok(())
}

quick_main!(run);