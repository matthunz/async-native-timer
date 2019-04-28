use crate::RawTimer;
use libc::{timerfd_create, CLOCK_MONOTONIC, TFD_NONBLOCK};
use mio::unix::EventedFd;
use mio::{Evented, Poll, PollOpt, Ready, Token};
use std::io;
use std::os::unix::io::RawFd;
use std::time::Duration;


pub struct UnixTimer {
    fd: RawFd,
}

impl Evented for UnixTimer {
    fn register(
        &self,
        poll: &Poll,
        token: Token,
        interest: Ready,
        opts: PollOpt,
    ) -> io::Result<()> {
        EventedFd(&self.fd).register(poll, token, interest, opts)
    }
    fn reregister(
        &self,
        poll: &Poll,
        token: Token,
        interest: Ready,
        opts: PollOpt,
    ) -> io::Result<()> {
        EventedFd(&self.fd).reregister(poll, token, interest, opts)
    }
    fn deregister(&self, poll: &Poll) -> io::Result<()> {
        EventedFd(&self.fd).deregister(poll)
    }
}

impl RawTimer for UnixTimer {
    fn new_timer() -> Self {
        let fd = unsafe { timerfd_create(CLOCK_MONOTONIC, TFD_NONBLOCK) };
        Self { fd }
    }
    fn set_timer(&mut self, time: Duration) {
        let it_value = libc::timespec {
            tv_sec: time.as_secs() as libc::time_t,
            tv_nsec: time.subsec_nanos() as libc::suseconds_t,
        };

        let timer = libc::itimerspec {
            it_interval: unsafe { std::mem::zeroed() },
            it_value,
        };
        let ret = unsafe { libc::timerfd_settime(self.fd, 0, &timer, std::ptr::null_mut()) };
        assert_eq!(ret, 0);
    }
}
/*
#[cfg(test)]
mod tests {
    use super::*;

    use mio::Events;
    use std::time::Instant;
    #[test]
    fn it_works() {
        let mut timer = UnixTimer::new_timer();

        let time = Duration::from_secs(2);
        let start = Instant::now();
        timer.set_timer(time);

        let mut events = Events::with_capacity(10);
        let poll = Poll::new().unwrap();
        poll.register(&timer, Token(0), Ready::all(), PollOpt::edge())
            .unwrap();

        poll.poll(&mut events, None).unwrap();
        let elapsed = Instant::now() - start;
        let err = Duration::from_millis(5);
        assert!(elapsed < (time + err) && elapsed > (time - err));
    }
}
*/