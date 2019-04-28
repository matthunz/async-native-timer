use mio::Evented;
use std::time::Duration;


#[cfg(unix)]
#[path = "unix.rs"]
pub mod unix;
pub use unix::UnixTimer;

pub trait RawTimer: Evented {
    fn new_timer() -> Self;
    fn set_timer(&mut self, time: Duration);
}
