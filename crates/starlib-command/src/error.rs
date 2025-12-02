use starlib_network::NetworkError;

define_error! {
  #[kind(CommandErrorKind)]
  pub enum CommandError {
    Agent,
    Replay,
    Blocked,
    Network,
  }
}

impl From<NetworkError> for CommandError {
  #[inline]
  fn from(other: NetworkError) -> Self {
    Self::new(other, CommandErrorKind::Network)
  }
}
