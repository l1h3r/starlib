use std::net::TcpStream;
use tungstenite::client;
use tungstenite::protocol;
use tungstenite::protocol::WebSocketConfig;

use crate::builder::Builder;
use crate::traits::Decode;
use crate::traits::Encode;

type Socket = protocol::WebSocket<TcpStream>;

// -----------------------------------------------------------------------------
// Network Error
// -----------------------------------------------------------------------------

define_error! {
  #[kind(NetworkErrorKind)]
  pub enum NetworkError {
    Connect,
    Protobuf,
    Handshake,
    WebSocket,
  }
}

// -----------------------------------------------------------------------------
// Web Socket
// -----------------------------------------------------------------------------

#[derive(Debug)]
pub struct WebSocket(Socket);

impl WebSocket {
  #[inline]
  pub fn builder() -> Builder {
    Builder::new()
  }

  #[inline]
  pub(crate) fn connect(domain: &str, stream: TcpStream) -> Result<Self, NetworkError> {
    client::client_with_config(domain, stream, Some(Self::config()))
      .map(|(socket, _)| Self(socket))
      .map_err(|error| NetworkError::new(error, NetworkErrorKind::Handshake))
  }

  #[inline]
  pub fn send<T>(&mut self, message: T) -> Result<(), NetworkError>
  where
    T: Encode,
  {
    self
      .0
      .send(T::encode(message))
      .map_err(|error| NetworkError::new(error, NetworkErrorKind::WebSocket))
  }

  #[inline]
  pub fn read<T>(&mut self) -> Result<T, NetworkError>
  where
    T: Decode,
  {
    self
      .0
      .read()
      .map_err(|error| NetworkError::new(error, NetworkErrorKind::WebSocket))
      .and_then(T::decode)
  }

  #[inline]
  pub fn read_into<T>(&mut self, buffer: &mut T) -> Result<(), NetworkError>
  where
    T: Decode,
  {
    self
      .0
      .read()
      .map_err(|error| NetworkError::new(error, NetworkErrorKind::WebSocket))
      .and_then(|message| T::decode_into(buffer, message))
  }

  #[inline]
  pub fn write<T>(&mut self, message: T) -> Result<(), NetworkError>
  where
    T: Encode,
  {
    self
      .0
      .write(T::encode(message))
      .map_err(|error| NetworkError::new(error, NetworkErrorKind::WebSocket))
  }

  #[inline]
  pub fn flush(&mut self) -> Result<(), NetworkError> {
    self
      .0
      .flush()
      .map_err(|error| NetworkError::new(error, NetworkErrorKind::WebSocket))
  }

  #[inline]
  pub(crate) fn config() -> WebSocketConfig {
    WebSocketConfig::default()
      .accept_unmasked_frames(false)
      .max_frame_size(Some(128 << 20)) // 128MiB
      .max_message_size(Some(32 << 20)) // 32MiB
      .max_write_buffer_size(usize::MAX)
      .read_buffer_size(128 << 10) // 128KiB
      .write_buffer_size(128 << 10) // 128KiB
  }
}
