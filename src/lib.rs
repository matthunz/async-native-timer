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
    fn set_timer(&mut self, time: Duration, interval: Option<Duration>);
}

pub struct Delay<R: RawTimer> {
    evented: PollEvented<R>,
}

impl Delay<NativeTimer> {
    pub fn new(time: Duration) -> Self {
        Self::from_raw(NativeTimer::new_timer(), time)
    }
}

impl<R: RawTimer> Delay<R> {
    pub fn from_raw(mut raw: R, time: Duration) -> Self {
        raw.set_timer(time, None);
        Self {
            evented: PollEvented::new(raw),
        }
    }
}

impl<R: RawTimer + Evented> Future for Delay<R> {
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
    use futures::executor::block_on;
    use std::time::Instant;

    #[test]
    fn it_works() {
        let time = Duration::from_secs(1);
        let timer = Delay::new(time);
        let start = Instant::now();

        block_on(timer);

        let elapsed = Instant::now() - start;
        let err = Duration::from_millis(1);
        assert!(elapsed < (time + err) && elapsed > (time - err));
    }
}