#![recursion_limit = "1024"]
#[macro_use]
extern crate error_chain;
mod errors {
    error_chain!{}
}
use errors::*;

extern crate state_machine_future;

//extern crate env_logger;

#[macro_use]
extern crate log;
extern crate fern;

extern crate hex;
extern crate time;

#[macro_use]
extern crate btc;

//use chrono::prelude::*;
//use chrono::Local;

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

extern crate chrono;
//use chrono::prelude::*;
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;

use std::net::SocketAddr;
use std::thread;
use std::sync::{Arc, Mutex};
//use log::LevelFilter;


#[derive(StructOpt, Debug)]
#[structopt(name = "")]
/// Node general settings
pub struct EnvVar {
  #[structopt(long = "node-socket-addr", default_value = "127.0.0.1:8333")]
  node_addr: SocketAddr,

  #[structopt(long = "admin-socket-addr", default_value = "127.0.0.1:8081")]
  admin_addr: SocketAddr,

  #[structopt(long = "log-file", default_value = "output.log")]
  log_file: String,
}

fn process_peer(socket: TcpStream, tx_sched: Arc<Mutex<commons::TxMpscMainToSched>>) {
    i!("New peer connection: {:?}", socket.peer_addr().unwrap());
    let (tx_peer, rx_peer) = mpsc::unbounded();
    let (tx_toolbox, rx_toolbox) = mpsc::unbounded();
    {
        d!("after channel mpsc created.");
        let tx_sched_unlocked = tx_sched.lock().unwrap(); // TODO may error
        d!("After mutex was locked.");
        tx_sched_unlocked.unbounded_send(
            Box::new(
                commons::MainToSchedRequestContent::Register(
                    commons::RxPeers(socket.peer_addr().unwrap(), rx_peer.into_future()),
                    tx_toolbox,
                )
            )
        ).unwrap(); // TODO may error
        d!("After tx_sched send");
    }

    let peer = btc::peer::Peer::new(socket, tx_peer, tx_sched, rx_toolbox);
    let peer_machina = btc::peer::machina::Machina::start(peer)
        .map_err(|_| ())
        .map(|_| ());

    tokio::spawn(peer_machina);
}
fn process_admin(socket: TcpStream, tx_sched: Arc<Mutex<commons::TxMpscMainToSched>>) {
    i!("New admin connection: {:?}", socket.peer_addr().unwrap());
    let (tx_peer, rx_peer) = mpsc::unbounded();
    let (tx_toolbox, rx_toolbox) = mpsc::unbounded();
    {
        d!("after channel mpsc created.");
        let tx_sched_unlocked = tx_sched.lock().unwrap(); // TODO may error
        d!("After mutex was locked.");
        tx_sched_unlocked.unbounded_send(
            Box::new(
                commons::MainToSchedRequestContent::Register(
                    commons::RxPeers(socket.peer_addr().unwrap(), rx_peer.into_future()),
                    tx_toolbox,
                )
            )
        ).unwrap(); // TODO may error
        d!("After tx_sched send");
    }

    let peer = btc::admin::Peer::new(socket, tx_peer, tx_sched, rx_toolbox);
    let peer_machina = btc::admin::machina::Machina::start(peer)
        .map_err(|_| ())
        .map(|_| ());

    tokio::spawn(peer_machina);
}

//use env_logger::LogBuilder;

fn run() -> Result<()> {
    let args = EnvVar::from_args();

    fern::Dispatch::new()
        // Perform allocation-free log formatting
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] [{}",
                chrono::Local::now().format("[%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        // Add blanket level filter -
        .level(log::LevelFilter::Info)
        // - and per-module overrides
        //.level_for("hyper", log::LevelFilter::Info)
        // Output to stdout, files, and other Dispatch configurations
        .chain(std::io::stdout())
        .chain(fern::log_file(args.log_file).unwrap())
        // Apply globally
        .apply()
        .unwrap();

    let (tx, rx) = mpsc::unbounded();
    let mtx = Arc::new(Mutex::new(tx));
    let scheduler = btc::exec::scheduler::Scheduler::new(rx, 3)
        .map_err(|_| ());
    thread::spawn(move || {
        tokio::run(scheduler);
    });

    let listener_peer = TcpListener::bind(&args.node_addr).unwrap();
    let listener_admin = TcpListener::bind(&args.admin_addr).unwrap();

    struct IsAdmin(bool);
    let server_listeners = listener_admin
        .incoming()
        .map(|socket| (socket, IsAdmin(true)))
        .select(listener_peer
            .incoming()
            .map(|socket| (socket, IsAdmin(false))))
        .for_each(move |(socket, IsAdmin(is_admin))| {
            if is_admin {
                process_admin(socket, Arc::clone(&mtx));
            } else {
                process_peer(socket, Arc::clone(&mtx));
            }
            Ok(())
        })
        .map_err(|err| {
            i!("accept error = {:?}", err);
        });



    i!("server_peer running on {:?}", args.node_addr);
    i!("server_admin running on {:?}", args.admin_addr);

    //tokio::run(server_peer);
    tokio::run(server_listeners);

    Ok(())
}

quick_main!(run);
