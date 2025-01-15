//! Async IO abstractions.

mod copy;
mod cursor;
mod empty;
mod read;
mod repeat;
mod seek;
mod stdio;
mod streams;
mod write;

pub use crate::runtime::AsyncPollable;
pub use copy::*;
pub use cursor::*;
pub use empty::*;
pub use read::*;
pub use repeat::*;
pub use seek::*;
pub use stdio::*;
pub use streams::*;
pub use write::*;

/// The error type for I/O operations.
///
pub use std::io::Error;

/// A specialized Result type for I/O operations.
///
pub use std::io::Result;
