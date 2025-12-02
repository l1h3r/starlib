use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::io;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use std::time::Instant;

use crate::retries::Interval;
use crate::retries::RetryStrategy;
use crate::socket::NetworkError;
use crate::socket::NetworkErrorKind;
use crate::socket::WebSocket;

// -----------------------------------------------------------------------------
// Builder
// -----------------------------------------------------------------------------

pub struct Builder {
  host: IpAddr,
  port: u16,
  conn_timeout: Option<Duration>,
  recv_timeout: Option<Duration>,
  send_timeout: Option<Duration>,
  retry_attempts: usize,
  retry_strategy: Box<dyn RetryStrategy>,
}

impl Builder {
  #[inline]
  pub fn new() -> Self {
    Self {
      host: IpAddr::V4(Ipv4Addr::LOCALHOST),
      port: 5000,
      conn_timeout: None,
      recv_timeout: None,
      send_timeout: None,
      retry_attempts: 15,
      retry_strategy: Box::new(Interval::new(Duration::from_millis(1000))),
    }
  }

  #[inline]
  pub fn host<T>(mut self, value: T) -> Self
  where
    T: Into<IpAddr>,
  {
    self.host = value.into();
    self
  }

  #[inline]
  pub const fn port(mut self, value: u16) -> Self {
    self.port = value;
    self
  }

  #[inline]
  pub const fn conn_timeout(mut self, value: Duration) -> Self {
    self.conn_timeout = Some(value);
    self
  }

  #[inline]
  pub const fn recv_timeout(mut self, value: Duration) -> Self {
    self.recv_timeout = Some(value);
    self
  }

  #[inline]
  pub const fn send_timeout(mut self, value: Duration) -> Self {
    self.send_timeout = Some(value);
    self
  }

  #[inline]
  pub const fn retry_attempts(mut self, value: usize) -> Self {
    self.retry_attempts = value;
    self
  }

  #[inline]
  pub fn retry_strategy<T>(mut self, value: T) -> Self
  where
    T: RetryStrategy + 'static,
  {
    self.retry_strategy = Box::new(value);
    self
  }

  pub fn connect(self) -> Result<WebSocket, NetworkError> {
    let socket: SocketAddr = (self.host, self.port).into();
    let domain: String = format!("wss://{socket}/sc2api");

    debug!("> Connecting to StarCraft II");

    debug!("> > Initializing TCP Stream");
    debug!("    {socket}");

    let stream: TcpStream = self.to_stream(socket)?;

    debug!("> > Initializing WebSocket");
    debug!("    {domain}");

    let socket: WebSocket = WebSocket::connect(domain.as_str(), stream)?;

    debug!("> Connected to StarCraft II");

    Ok(socket)
  }

  fn to_stream(&self, socket: SocketAddr) -> Result<TcpStream, NetworkError> {
    let instant: Instant = Instant::now();

    let mut attempts: usize = self.retry_attempts;
    let mut strategy: Box<dyn RetryStrategy> = dyn_clone::clone_box(&*self.retry_strategy);

    let stream: Result<TcpStream, io::Error> = loop {
      match self.try_connect(socket) {
        Ok(socket) => {
          break Ok(socket);
        }
        Err(error) if attempts == 0 || error.kind() != io::ErrorKind::ConnectionRefused => {
          break Err(error);
        }
        Err(_) => {
          let delay: Duration = strategy.tick();

          attempts -= 1;

          debug!(
            "> > Connection Retry #{attempt} in {timeout}ms",
            attempt = self.retry_attempts - attempts,
            timeout = delay.as_millis(),
          );

          thread::sleep(delay);
        }
      }
    };

    let elapsed: Duration = instant.elapsed();

    debug!(
      "> > Connection Took {}s:{}ms",
      elapsed.as_secs(),
      elapsed.subsec_millis()
    );

    stream
      .and_then(|stream| stream.set_nodelay(false).map(|()| stream))
      .and_then(|stream| stream.set_write_timeout(self.send_timeout).map(|()| stream))
      .and_then(|stream| stream.set_read_timeout(self.recv_timeout).map(|()| stream))
      .map_err(|error| NetworkError::new(error, NetworkErrorKind::Connect))
  }

  fn try_connect(&self, socket: SocketAddr) -> Result<TcpStream, io::Error> {
    if let Some(timeout) = self.conn_timeout {
      TcpStream::connect_timeout(&socket, timeout)
    } else {
      TcpStream::connect(socket)
    }
  }
}

impl Clone for Builder {
  #[inline]
  fn clone(&self) -> Self {
    Self {
      host: self.host,
      port: self.port,
      conn_timeout: self.conn_timeout,
      recv_timeout: self.recv_timeout,
      send_timeout: self.send_timeout,
      retry_attempts: self.retry_attempts,
      retry_strategy: dyn_clone::clone_box(&*self.retry_strategy),
    }
  }
}

impl Debug for Builder {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.debug_struct("Builder")
      .field("host", &self.host)
      .field("port", &self.port)
      .field("conn_timeout", &self.conn_timeout)
      .field("recv_timeout", &self.recv_timeout)
      .field("send_timeout", &self.send_timeout)
      .field("retry_attempts", &self.retry_attempts)
      .finish_non_exhaustive()
  }
}
