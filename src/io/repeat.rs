use super::AsyncRead;

pub struct Repeat(u8);
impl AsyncRead for Repeat {
    async fn read(&mut self, buf: &mut [u8]) -> super::Result<usize> {
        buf.fill(self.0);
        Ok(buf.len())
    }

    #[inline]
    fn as_repeated_value(&self) -> Option<u8> {
        Some(self.0)
    }
}

/// Creates a value that infinitely produces a given value.
pub fn repeat(byte: u8) -> Repeat {
    Repeat(byte)
}
