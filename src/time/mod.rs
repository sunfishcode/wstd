//! Async time interfaces.

pub(crate) mod utils;

mod duration;
mod instant;
pub use duration::Duration;
pub use instant::Instant;

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use wasi::clocks::{
    monotonic_clock::{subscribe_duration, subscribe_instant},
    wall_clock,
};

use crate::{
    iter::AsyncIterator,
    runtime::{AsyncPollable, Reactor},
};

/// A measurement of the system clock, useful for talking to external entities
/// like the file system or other processes.
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct SystemTime(wall_clock::Datetime);

impl SystemTime {
    pub fn now() -> Self {
        Self(wall_clock::now())
    }
}

/// An async iterator representing notifications at fixed interval.
pub fn interval(duration: Duration) -> Interval {
    Interval { duration }
}

/// An async iterator representing notifications at fixed interval.
///
/// See the [`interval`] function for more.
#[derive(Debug)]
pub struct Interval {
    duration: Duration,
}
impl AsyncIterator for Interval {
    type Item = Instant;

    async fn next(&mut self) -> Option<Self::Item> {
        Some(Timer::after(self.duration).await)
    }
}

#[derive(Debug)]
pub struct Timer(Option<AsyncPollable>);

impl Timer {
    pub fn never() -> Timer {
        Timer(None)
    }
    pub fn at(deadline: Instant) -> Timer {
        let pollable = Reactor::current().schedule(subscribe_instant(*deadline));
        Timer(Some(pollable))
    }
    pub fn after(duration: Duration) -> Timer {
        let pollable = Reactor::current().schedule(subscribe_duration(*duration));
        Timer(Some(pollable))
    }
    pub fn set_after(&mut self, duration: Duration) {
        *self = Self::after(duration);
    }
    pub async fn wait(&self) -> Instant {
        match &self.0 {
            Some(pollable) => pollable.wait_for().await,
            None => std::future::pending().await,
        }
        Instant::now()
    }
}

impl Future for Timer {
    type Output = Instant;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.as_ref();
        let pinned = std::pin::pin!(this.wait());
        pinned.poll(cx)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    async fn debug_duration(what: &str, f: impl Future<Output = Instant>) {
        let start = Instant::now();
        let now = f.await;
        let d = now.duration_since(start);
        let d: std::time::Duration = d.into();
        println!("{what} awaited for {} s", d.as_secs_f32());
    }

    #[test]
    fn timer_now() {
        crate::runtime::block_on(debug_duration("timer_now", async {
            Timer::at(Instant::now()).await
        }));
    }

    #[test]
    fn timer_after_100_milliseconds() {
        crate::runtime::block_on(debug_duration("timer_after_100_milliseconds", async {
            Timer::after(Duration::from_millis(100)).await
        }));
    }
}
