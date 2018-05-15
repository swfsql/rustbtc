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
