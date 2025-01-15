use alloc::format;
use alloc::string::String;
use wasi::{
    io::streams::{InputStream, OutputStream},
    sockets::tcp::TcpSocket,
};

use crate::io::{self, AsyncInputStream, AsyncOutputStream};

/// A TCP stream between a local and a remote socket.
pub struct TcpStream {
    input: AsyncInputStream,
    output: AsyncOutputStream,
    socket: TcpSocket,
}

impl TcpStream {
    pub(crate) fn new(input: InputStream, output: OutputStream, socket: TcpSocket) -> Self {
        TcpStream {
            input: AsyncInputStream::new(input),
            output: AsyncOutputStream::new(output),
            socket,
        }
    }
    /// Returns the socket address of the remote peer of this TCP connection.
    pub fn peer_addr(&self) -> io::Result<String> {
        let addr = self
            .socket
            .remote_address()
            .map_err(super::tcp_listener::to_io_err)?;
        Ok(format!("{addr:?}"))
    }

    pub fn split(&self) -> (ReadHalf<'_>, WriteHalf<'_>) {
        (ReadHalf(self), WriteHalf(self))
    }
}

impl Drop for TcpStream {
    fn drop(&mut self) {
        let _ = self.socket.shutdown(wasi::sockets::tcp::ShutdownType::Both);
    }
}

impl io::AsyncRead for TcpStream {
    async fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.input.read(buf).await
    }

    fn as_async_input_stream(&self) -> Option<&AsyncInputStream> {
        Some(&self.input)
    }
}

impl io::AsyncRead for &TcpStream {
    async fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.input.read(buf).await
    }

    fn as_async_input_stream(&self) -> Option<&AsyncInputStream> {
        (**self).as_async_input_stream()
    }
}

impl io::AsyncWrite for TcpStream {
    async fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.output.write(buf).await
    }

    async fn flush(&mut self) -> io::Result<()> {
        self.output.flush().await
    }

    fn as_async_output_stream(&self) -> Option<&AsyncOutputStream> {
        Some(&self.output)
    }
}

impl io::AsyncWrite for &TcpStream {
    async fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.output.write(buf).await
    }

    async fn flush(&mut self) -> io::Result<()> {
        self.output.flush().await
    }

    fn as_async_output_stream(&self) -> Option<&AsyncOutputStream> {
        (**self).as_async_output_stream()
    }
}

pub struct ReadHalf<'a>(&'a TcpStream);
impl<'a> io::AsyncRead for ReadHalf<'a> {
    async fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf).await
    }

    fn as_async_input_stream(&self) -> Option<&AsyncInputStream> {
        self.0.as_async_input_stream()
    }
}

impl<'a> Drop for ReadHalf<'a> {
    fn drop(&mut self) {
        let _ = self
            .0
            .socket
            .shutdown(wasi::sockets::tcp::ShutdownType::Receive);
    }
}

pub struct WriteHalf<'a>(&'a TcpStream);
impl<'a> io::AsyncWrite for WriteHalf<'a> {
    async fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf).await
    }

    async fn flush(&mut self) -> io::Result<()> {
        self.0.flush().await
    }

    fn as_async_output_stream(&self) -> Option<&AsyncOutputStream> {
        self.0.as_async_output_stream()
    }
}

impl<'a> Drop for WriteHalf<'a> {
    fn drop(&mut self) {
        let _ = self
            .0
            .socket
            .shutdown(wasi::sockets::tcp::ShutdownType::Send);
    }
}
