//! HTTP body types

use crate::io::{AsyncRead, Cursor, Empty};

pub use super::response::IncomingBody;

/// A trait representing an HTTP body.
#[doc(hidden)]
pub trait Body: AsyncRead {
    /// Returns the exact remaining length of the iterator, if known.
    fn len(&self) -> Option<usize>;

    /// Returns `true`` if the body is known to be empty.
    fn is_empty(&self) -> bool {
        matches!(self.len(), Some(0))
    }
}

/// Conversion into a `Body`.
#[doc(hidden)]
pub trait IntoBody {
    /// What type of `Body` are we turning this into?
    type IntoBody: Body;
    /// Convert into `Body`.
    fn into_body(self) -> Self::IntoBody;
}
impl<T> IntoBody for T
where
    T: Body,
{
    type IntoBody = T;
    fn into_body(self) -> Self::IntoBody {
        self
    }
}

impl IntoBody for String {
    type IntoBody = BoundedBody<Vec<u8>>;
    fn into_body(self) -> Self::IntoBody {
        BoundedBody(Cursor::new(self.into_bytes()))
    }
}

impl IntoBody for &str {
    type IntoBody = BoundedBody<Vec<u8>>;
    fn into_body(self) -> Self::IntoBody {
        BoundedBody(Cursor::new(self.to_owned().into_bytes()))
    }
}

impl IntoBody for Vec<u8> {
    type IntoBody = BoundedBody<Vec<u8>>;
    fn into_body(self) -> Self::IntoBody {
        BoundedBody(Cursor::new(self))
    }
}

impl IntoBody for &[u8] {
    type IntoBody = BoundedBody<Vec<u8>>;
    fn into_body(self) -> Self::IntoBody {
        BoundedBody(Cursor::new(self.to_owned()))
    }
}

/// An HTTP body with a known length
#[derive(Debug)]
pub struct BoundedBody<T>(Cursor<T>);

impl<T: AsRef<[u8]>> AsyncRead for BoundedBody<T> {
    async fn read(&mut self, buf: &mut [u8]) -> crate::io::Result<usize> {
        self.0.read(buf).await
    }
}
impl<T: AsRef<[u8]>> Body for BoundedBody<T> {
    fn len(&self) -> Option<usize> {
        Some(self.0.get_ref().as_ref().len())
    }
}

impl Body for Empty {
    fn len(&self) -> Option<usize> {
        Some(0)
    }
}
