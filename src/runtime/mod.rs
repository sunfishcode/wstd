//! Async event loop support.
//!
//! The way to use this is to call [`block_on()`]. Inside the future, [`Reactor::current`]
//! will give an instance of the [`Reactor`] running the event loop, which can be
//! to [`AsyncPollable::wait_for`] instances of
//! [`wasi::Pollable`](https://docs.rs/wasi/latest/wasi/io/poll/struct.Pollable.html).
//! This will automatically wait for the futures to resolve, and call the
//! necessary wakers to work.

#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, unreachable_pub)]

mod block_on;
mod reactor;

pub use block_on::block_on;
use core::cell::RefCell;
pub use reactor::{AsyncPollable, Reactor, WaitFor};

// There are no threads in WASI 0.2, so this is just a safe way to thread a single reactor to all
// use sites in the background.
std::thread_local! {
pub(crate) static REACTOR: RefCell<Option<Reactor>> = RefCell::new(None);
}
