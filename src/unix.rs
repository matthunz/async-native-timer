use crate::RawTimer;
use libc::{timerfd_create, CLOCK_MONOTONIC, TFD_NONBLOCK};
use mio::unix::EventedFd;
use mio::{Evented, Poll, PollOpt, Ready, Token};
use std::io;
use std::os::unix::io::RawFd;
use std::time::Duration;


/// Unix implementation of a `RawTimer` based on file descriptors
pub struct NativeTimer {
    fd: RawFd,
}

impl Evented for NativeTimer {
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

fn as_timespec(time: Duration) -> libc::timespec {
    libc::timespec {
        tv_sec: time.as_secs() as libc::time_t,
        tv_nsec: time.subsec_nanos() as libc::suseconds_t,
    }
}

impl RawTimer for NativeTimer {
    fn new_timer() -> Self {
        let fd = unsafe { timerfd_create(CLOCK_MONOTONIC, TFD_NONBLOCK) };
        NativeTimer { fd }
    }
    fn set_timer(&mut self, time: Duration, interval: Option<Duration>) {
        let it_interval = match interval {
            Some(dur) => as_timespec(dur),
            None => unsafe { std::mem::zeroed() },
        };
        let timer = libc::itimerspec {
            it_value: as_timespec(time),
            it_interval,
        };

        let ret = unsafe { libc::timerfd_settime(self.fd, 0, &timer, std::ptr::null_mut()) };
        assert_eq!(ret, 0);
    }
}
