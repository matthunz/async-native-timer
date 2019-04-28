use mio::Evented;
use romio::raw::PollEvented;
use std::future::Future;
use std::pin::Pin;
use std::task;
use std::time::Duration;

#[cfg(unix)]
#[path = "unix.rs"]
pub mod raw;

pub use raw::NativeTimer;


pub trait RawTimer: Evented {
    fn new_timer() -> Self;
    fn set_timer(&mut self, time: Duration);
}

pub struct Timer<R: RawTimer> {
    evented: PollEvented<R>,
}

impl Timer<NativeTimer> {
    pub fn new() -> Self {
        Self::from_raw(NativeTimer::new_timer())
    }
}

impl<R: RawTimer> Timer<R> {
    pub fn from_raw(raw: R) -> Self {
        Self {
            evented: PollEvented::new(raw),
        }
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


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let timer = Timer::new();
    }
}