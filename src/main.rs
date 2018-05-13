#![recursion_limit = "1024"]
#![feature(box_patterns)]
#[macro_use]
extern crate error_chain;
mod errors {
    error_chain!{
        // links {
        //     Btc(::btc::errors::Error)
        // }
    }
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
use btc::actor::commons;
use futures::sync::{mpsc,oneshot};
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
use std::sync::{Arc, Mutex};
use std::thread;
//use log::LevelFilter;

#[derive(StructOpt, Debug)]
#[structopt(name = "")]
/// Node general settings
pub struct EnvVar {
    #[structopt(long = "node-socket-addr", default_value = "127.0.0.1:8333")]
    node_addr: SocketAddr,

    #[structopt(long = "admin-socket-addr", default_value = "127.0.0.1:8081")]
    admin_addr: SocketAddr,

    #[structopt(long = "public-socket-addr", default_value = "127.0.0.1:8333")]
    public_addr: SocketAddr,

    #[structopt(long = "log-file", default_value = "output.log")]
    log_file: String,
}

fn process_peer(socket: TcpStream, tx_sched: Arc<Mutex<commons::TxMpscMainToSched>>) {
    i!(
        "New peer connection: {:?}",
        socket.peer_addr().expect(&ff!())
    );
    let (tx_peer, rx_peer) = mpsc::unbounded();
    let (tx_router, rx_router) = mpsc::unbounded();
    let (otx, orx) = oneshot::channel::<Box<commons::SchedulerResponse>>();
    {
        d!("after channel mpsc created.");
        let tx_sched_unlocked = tx_sched.lock().expect(&ff!()); // TODO may error
        d!("After mutex was locked.");
        tx_sched_unlocked
            .unbounded_send(Box::new(commons::MainToSchedRequestContent::Register(
                socket.peer_addr().expect(&ff!()), 
                rx_peer.into_future(),
                tx_router,
                otx,
            )))
            .expect(&ff!()); // TODO may error
        d!("After tx_sched send");
    }//

    let shot_back = orx.wait().expect(&ff!()); // TODO async
    let actor_id = {
        if let box commons::SchedulerResponse::RegisterResponse(Ok(ref res_actor_id)) = shot_back {
            res_actor_id.clone()
        } else {
            panic!("TODO: error when registering new peer");
        }
    };

    // TODO: 
    // let tx_sched_inner = tx_sched.lock().unwrap().clone();

    let peer = btc::actor::peer::Peer::new(socket, tx_peer, tx_sched, rx_router, actor_id);
    let peer_machina = btc::actor::peer::machina::Machina::start(peer)
        .map_err(|_| ())
        .map(|_| ());

    tokio::spawn(peer_machina);
}
fn process_admin(socket: TcpStream, tx_sched: Arc<Mutex<commons::TxMpscMainToSched>>) {
    i!(
        "New admin connection: {:?}",
        socket.peer_addr().expect(&ff!())
    );
    let (tx_peer, rx_peer) = mpsc::unbounded();
    let (tx_router, rx_router) = mpsc::unbounded();
    let (otx, orx) = oneshot::channel::<Box<commons::SchedulerResponse>>();
    {
        d!("after channel mpsc created.");
        let tx_sched_unlocked = tx_sched.lock().expect(&ff!()); // TODO may error
        d!("After mutex was locked.");
        tx_sched_unlocked
            .unbounded_send(Box::new(commons::MainToSchedRequestContent::Register(
                socket.peer_addr().expect(&ff!()),
                rx_peer.into_future(),
                tx_router,
                otx,
            )))
            .expect(&ff!()); // TODO may error
        d!("After tx_sched send");
    }

    let shot_back = orx.wait().expect(&ff!()); // TODO async
    let actor_id = {
        if let box commons::SchedulerResponse::RegisterResponse(Ok(ref res_actor_id)) = shot_back {
            res_actor_id.clone()
        } else {
            panic!("TODO: error when registering new peer");
        }
    };

    let peer = btc::actor::admin::Peer::new(socket, tx_peer, tx_sched, rx_router, actor_id);
    let peer_machina = btc::actor::admin::machina::Machina::start(peer)
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
        .level(log::LevelFilter::Debug)
        // - and per-module overrides
        //.level_for("hyper", log::LevelFilter::Info)
        // Output to stdout, files, and other Dispatch configurations
        .chain(std::io::stdout())
        .chain(fern::log_file(args.log_file).expect(&ff!()))
        // Apply globally
        .apply()
        .expect(&ff!());

    let (tx, rx) = mpsc::unbounded();
    let (tx_peer_messenger_reg, rx_peer_messenger_reg) = mpsc::unbounded();
    let (tx_worker_to_router_backup, rx_worker_to_router) = mpsc::unbounded();
    let (tx_bchain_to_sched, rx_bchain_to_sched) = mpsc::unbounded();
    let (tx_worker_to_bchain, rx_worker_to_bchain) = mpsc::unbounded();
    let mtx = Arc::new(Mutex::new(tx));

    let router = btc::actor::router::Router::new(rx_peer_messenger_reg, rx_worker_to_router)
        .map_err(|_| ());

    //impl future/state machine
    // let bchain = btc::actor::blockchain::Blockchain::new(tx_bchain_to_sched, rx_worker_to_bchain)
    //     .map_err(|_| ());
    let scheduler = btc::actor::scheduler::Scheduler::new(rx, tx_peer_messenger_reg, tx_worker_to_router_backup, rx_bchain_to_sched.into_future(), tx_worker_to_bchain, 3)
        .map_err(|_| ());

    thread::spawn(move || {
        tokio::run(router);
    });
    thread::spawn(move || tokio::run(scheduler));

    let listener_peer = TcpListener::bind(&args.node_addr).expect(&ff!());
    let listener_admin = TcpListener::bind(&args.admin_addr).expect(&ff!());

    struct IsAdmin(bool);
    let server_listeners = listener_admin
        .incoming()
        .map(|socket| (socket, IsAdmin(true)))
        .select(
            listener_peer
                .incoming()
                .map(|socket| (socket, IsAdmin(false))),
        )
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
