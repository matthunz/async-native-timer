use mio::Evented;
use romio::raw::PollEvented;
use std::future::Future;
use std::pin::Pin;
use std::task;
use std::time::Duration;


#[cfg(unix)]
#[path = "unix.rs"]
pub mod unix;
pub use unix::UnixTimer;


pub trait RawTimer: Evented {
    fn new_timer() -> Self;
    fn set_timer(&mut self, time: Duration);
}

pub struct Timer<R: RawTimer> {
    evented: PollEvented<R>,
}

impl<R: RawTimer> Timer<R> {
    pub fn new(raw: R) -> Self {
        let evented = PollEvented::new(raw);
        Self { evented }
    }
}

impl<R: RawTimer + Evented> Future for Timer<R> {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<()> {
        Pin::new(&mut self.evented)
            .poll_read_ready(cx)
            .map(|_res| ())
    }
}
