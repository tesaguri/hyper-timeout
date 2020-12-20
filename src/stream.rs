use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration};

use hyper::client::connect::{Connected, Connection};
use pin_project::pin_project;
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use tokio_io_timeout::TimeoutStream;

/// A timeout stream that implements required traits to be a Connector
#[pin_project]
#[derive(Debug)]
pub struct TimeoutConnectorStream<S>(#[pin] TimeoutStream<S>);

impl<S> TimeoutConnectorStream<S>
where
    S: AsyncRead + AsyncWrite,
{
    /// Returns a new `TimeoutConnectorStream` wrapping the specified stream.
    ///
    /// There is initially no read or write timeout.
    pub fn new(stream: TimeoutStream<S>) -> TimeoutConnectorStream<S> {
        TimeoutConnectorStream(stream)
    }

    /// Returns the current read timeout.
    pub fn read_timeout(&self) -> Option<Duration> {
        self.0.read_timeout()
    }

    /// Sets the read timeout.
    ///
    /// This will reset any pending read timeout.
    pub fn set_read_timeout(&mut self, timeout: Option<Duration>) {
        self.0.set_read_timeout(timeout)
    }

    /// Returns the current write timeout.
    pub fn write_timeout(&self) -> Option<Duration> {
        self.0.write_timeout()
    }

    /// Sets the write timeout.
    ///
    /// This will reset any pending write timeout.
    pub fn set_write_timeout(&mut self, timeout: Option<Duration>) {
        self.0.set_write_timeout(timeout)
    }

    /// Returns a shared reference to the inner stream.
    pub fn get_ref(&self) -> &S {
        self.0.get_ref()
    }

    /// Returns a mutable reference to the inner stream.
    pub fn get_mut(&mut self) -> &mut S {
        self.0.get_mut()
    }

    /// Returns a pinned mutable reference to the inner stream.
    pub fn get_pin_mut(self: Pin<&mut Self>) -> Pin<&mut S> {
        self.project().0.get_pin_mut()
    }

    /// Consumes the stream, returning the inner stream.
    pub fn into_inner(self) -> S {
        self.0.into_inner()
    }
}

impl<S> AsyncRead for TimeoutConnectorStream<S>
where
    S: AsyncRead + AsyncWrite,
{
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<Result<(), io::Error>> {
        self.project().0.poll_read(cx, buf)
    }
}

impl<S> AsyncWrite for TimeoutConnectorStream<S>
where
    S: AsyncRead + AsyncWrite,
{
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &[u8],
    ) -> Poll<Result<usize, io::Error>> {
        self.project().0.poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), io::Error>> {
        self.project().0.poll_flush(cx)
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), io::Error>> {
        self.project().0.poll_shutdown(cx)
    }
}

impl<S> Connection for TimeoutConnectorStream<S>
where
    S: AsyncRead + AsyncWrite + Connection,
{
    fn connected(&self) -> Connected {
        self.0.get_ref().connected()
    }
}
