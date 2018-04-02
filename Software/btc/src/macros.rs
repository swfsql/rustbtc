#[macro_export]

macro_rules! e {
    () => (error!("{}:{}] ", file!()[3..].to_string(), line!()));
    ($fmt:expr) => (error!("{}:{}] {}", file!()[3..].to_string(), line!(), $fmt));
    ($fmt:expr, $($arg:tt)*) => (error!("{}:{}] {}", file!()[3..].to_string(), line!(), $fmt, $($arg)*));
}

#[macro_export]

macro_rules! w {
    () => (warn!("{}:{}] ", file!()[3..].to_string(), line!()));
    ($fmt:expr) => (warn!("{}:{}] {}", file!()[3..].to_string(), line!(), $fmt));
    ($fmt:expr, $($arg:tt)*) => (warn!("{}:{}] {}", file!()[3..].to_string(), line!(), $fmt, $($arg)*));
}

#[macro_export]
macro_rules! i {
    () => (info!("{}:{}] ", file!()[3..].to_string(), line!()));
    ($fmt:expr) => (info!("{}:{}] {}", file!()[3..].to_string(), line!(), $fmt));
    ($fmt:expr, $($arg:tt)*) => (info!("{}:{}] {}", file!()[3..].to_string(), line!(), $fmt, $($arg)*));
}

#[macro_export]
macro_rules! d {
    () => (debug!("{}:{}] ", file!()[3..].to_string(), line!()));
    ($fmt:expr) => (debug!("{}:{}] {}", file!()[3..].to_string(), line!(), $fmt));
    ($fmt:expr, $($arg:tt)*) => (debug!("{}:{}] {}", file!()[3..].to_string(), line!(), $fmt, $($arg)*));
}