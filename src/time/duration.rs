use super::Instant;
use crate::task::Sleep;
use std::future::IntoFuture;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use wasi::clocks::monotonic_clock;

/// A Duration type to represent a span of time, typically used for system
/// timeouts.
///
/// This type wraps `std::time::Duration` so we can implement traits on it
/// without coherence issues, just like if we were implementing this in the
/// stdlib.
#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash, Clone, Copy)]
pub struct Duration(pub(crate) monotonic_clock::Duration);
impl Duration {
    /// Creates a new `Duration` from the specified number of whole seconds and
    /// additional nanoseconds.
    #[must_use]
    #[inline]
    pub fn new(secs: u64, nanos: u32) -> Duration {
        std::time::Duration::new(secs, nanos).into()
    }

    /// Creates a new `Duration` from the specified number of whole seconds.
    #[must_use]
    #[inline]
    pub fn from_secs(secs: u64) -> Duration {
        std::time::Duration::from_secs(secs).into()
    }

    /// Creates a new `Duration` from the specified number of milliseconds.
    #[must_use]
    #[inline]
    pub fn from_millis(millis: u64) -> Self {
        std::time::Duration::from_millis(millis).into()
    }

    /// Creates a new `Duration` from the specified number of microseconds.
    #[must_use]
    #[inline]
    pub fn from_micros(micros: u64) -> Self {
        std::time::Duration::from_micros(micros).into()
    }

    /// Creates a new `Duration` from the specified number of seconds represented
    /// as `f64`.
    ///
    /// # Panics
    /// This constructor will panic if `secs` is not finite, negative or overflows `Duration`.
    ///
    /// # Examples
    /// ```no_run
    /// use wstd::time::Duration;
    ///
    /// let dur = Duration::from_secs_f64(2.7);
    /// assert_eq!(dur, Duration::new(2, 700_000_000));
    /// ```
    #[must_use]
    #[inline]
    pub fn from_secs_f64(secs: f64) -> Duration {
        std::time::Duration::from_secs_f64(secs).into()
    }

    /// Creates a new `Duration` from the specified number of seconds represented
    /// as `f32`.
    ///
    /// # Panics
    /// This constructor will panic if `secs` is not finite, negative or overflows `Duration`.
    #[must_use]
    #[inline]
    pub fn from_secs_f32(secs: f32) -> Duration {
        std::time::Duration::from_secs_f32(secs).into()
    }
}

impl std::ops::Deref for Duration {
    type Target = monotonic_clock::Duration;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Duration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<std::time::Duration> for Duration {
    fn from(inner: std::time::Duration) -> Self {
        Self(
            inner
                .as_nanos()
                .try_into()
                .expect("only dealing with durations that can fit in u64"),
        )
    }
}

impl Into<std::time::Duration> for Duration {
    fn into(self) -> std::time::Duration {
        std::time::Duration::from_nanos(self.0)
    }
}

impl Add<Duration> for Duration {
    type Output = Self;

    fn add(self, rhs: Duration) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign<Duration> for Duration {
    fn add_assign(&mut self, rhs: Duration) {
        *self = Self(self.0 + rhs.0)
    }
}

impl Sub<Duration> for Duration {
    type Output = Self;

    fn sub(self, rhs: Duration) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign<Duration> for Duration {
    fn sub_assign(&mut self, rhs: Duration) {
        *self = Self(self.0 - rhs.0)
    }
}

impl IntoFuture for Duration {
    type Output = Instant;

    type IntoFuture = Sleep;

    fn into_future(self) -> Self::IntoFuture {
        crate::task::sleep(self)
    }
}
