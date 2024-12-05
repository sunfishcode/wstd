//! Async IO abstractions.

mod copy;
mod cursor;
mod empty;
mod read;
mod seek;
mod stdio;
mod streams;
mod write;

pub use crate::runtime::AsyncPollable;
pub use copy::*;
pub use cursor::*;
pub use empty::*;
pub use read::*;
pub use seek::*;
pub use stdio::*;
pub use streams::*;
pub use write::*;

#[cfg(not(feature = "std"))]
use no_std_io2::io as std_io;
#[cfg(feature = "std")]
use std::io as std_io;

/// The error type for I/O operations.
///
pub use std_io::{Error, ErrorKind};

/// A specialized Result type for I/O operations.
///
pub use std_io::Result;
