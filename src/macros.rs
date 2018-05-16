
#[macro_export]
macro_rules! ok_some {
    ($e:expr) => {
        match $e {
            // Ok(Async::Ready(t)) => Some(t),
            Ok(Async::Ready(Some(t))) => Some(t),
            Ok(Async::NotReady) => None,
            Ok(Async::Ready(None)) => bail!("aborted"),
            Err(e) => bail!("Error on ok_ready: {:?}", e), //Err(From::from(e)),
        }
    };
}

#[macro_export]
macro_rules! worker_request_wrapped {
    ($state_peer:expr, $wr:expr, $priority:expr) => {{
    let wrp = WorkerRequestPriority($wr, $priority);
    let (otx, orx) = oneshot::channel::<Result<Box<WorkerResponseContent>>>();
    let actor_id = $state_peer.actor_id;
    let addr = AddrReqId(actor_id, $state_peer.next_request_counter());
    let wrc = WorkerRequestContent(wrp, otx, addr);
    $state_peer.tx_req.unbounded_send(Box::new(wrc))
        .expect(&ff!());
    ($state_peer, orx)
    }
}}

#[macro_export]
macro_rules! worker_request {
    ($state_peer:expr, $wr:expr, $priority:expr) => {{
        let (st_peer, orx) = worker_request_wrapped!($state_peer, $wr, $priority);
        (st_peer, orx.and_then(|i| Ok(i.expect(&ff!()).0)))
    }
}}

#[macro_export]
macro_rules! cf {
    () =>(|| {format!("[{}:{}] ",file!()[3..].to_string(),line!())});
    ($fmt:expr) =>(|| {format!(concat!("[{}:{}] ",$fmt),file!()[3..].to_string(),line!())});
    ($fmt:expr, $($arg:tt)*) =>(|| {format!(concat!("[{}:{}] ",$fmt),file!()[3..].to_string(),line!(),$($arg)*)});
}

#[macro_export]
macro_rules! ff {
    () =>(format!("[{}:{}] ",file!()[3..].to_string(),line!()));
    ($fmt:expr) =>(format!(concat!("[{}:{}] ",$fmt),file!()[3..].to_string(),line!()));
    ($fmt:expr, $($arg:tt)*) =>(format!(concat!("[{}:{}] ",$fmt),file!()[3..].to_string(),line!(),$($arg)*));
}

#[macro_export]
macro_rules! e {
    () =>(error!("{}:{}] ",file!()[3..].to_string(),line!()));
    ($fmt:expr) =>(error!(concat!("{}:{}] ",$fmt),file!()[3..].to_string(),line!()));
    ($fmt:expr, $($arg:tt)*) =>(error!(concat!("{}:{}] ",$fmt),file!()[3..].to_string(),line!(),$($arg)*));
}

#[macro_export]
macro_rules! w {
    () =>(warn!("{}:{}] ",file!()[3..].to_string(),line!()));
    ($fmt:expr) =>(warn!(concat!("{}:{}] ",$fmt),file!()[3..].to_string(),line!()));
    ($fmt:expr, $($arg:tt)*) =>(warn!(concat!("{}:{}] ",$fmt),file!()[3..].to_string(),line!(),$($arg)*));
}

#[macro_export]
macro_rules! i {
    () =>(info!("{}:{}] ",file!()[3..].to_string(),line!()));
    ($fmt:expr) =>(info!(concat!("{}:{}] ",$fmt),file!()[3..].to_string(),line!()));
    ($fmt:expr, $($arg:tt)*) =>(info!(concat!("{}:{}] ",$fmt),file!()[3..].to_string(),line!(),$($arg)*));
}

#[macro_export]
macro_rules! d {
    () =>(debug!("{}:{}] ",file!()[3..].to_string(),line!()));
    ($fmt:expr) =>(debug!(concat!("{}:{}] ",$fmt),file!()[3..].to_string(),line!()));
    ($fmt:expr, $($arg:tt)*) =>(debug!(concat!("{}:{}] ",$fmt),file!()[3..].to_string(),line!(),$($arg)*));
}

#[macro_export]
macro_rules! t {
    () =>(trace!("{}:{}] ",file!()[3..].to_string(),line!()));
    ($fmt:expr) =>(trace!(concat!("{}:{}] ",$fmt),file!()[3..].to_string(),line!()));
    ($fmt:expr, $($arg:tt)*) =>(trace!(concat!("{}:{}] ",$fmt),file!()[3..].to_string(),line!(),$($arg)*));
}


