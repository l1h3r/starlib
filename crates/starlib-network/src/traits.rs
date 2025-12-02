use tungstenite::Message;

use starlib_proto::Message as ProtoMessage;

use crate::socket::NetworkError;
use crate::socket::NetworkErrorKind;

// -----------------------------------------------------------------------------
// Seal
// -----------------------------------------------------------------------------

mod private {
  pub trait Sealed {}
}

impl<T> private::Sealed for T where T: ProtoMessage {}

// -----------------------------------------------------------------------------
// Encode Message
// -----------------------------------------------------------------------------

pub trait Encode: private::Sealed {
  fn encode(self) -> Message;
}

impl<T> Encode for T
where
  T: ProtoMessage,
{
  #[inline]
  fn encode(self) -> Message {
    Message::binary(self.encode_to_vec())
  }
}

// -----------------------------------------------------------------------------
// Decode Message
// -----------------------------------------------------------------------------

pub trait Decode: private::Sealed + Sized {
  fn decode(message: Message) -> Result<Self, NetworkError>;

  fn decode_into(this: &mut Self, message: Message) -> Result<(), NetworkError>;
}

impl<T> Decode for T
where
  T: ProtoMessage + Default,
{
  #[inline]
  fn decode(message: Message) -> Result<Self, NetworkError> {
    let mut this: Self = Self::default();
    Self::decode_into(&mut this, message).map(|()| this)
  }

  #[inline]
  fn decode_into(this: &mut Self, message: Message) -> Result<(), NetworkError> {
    if let Err(error) = T::merge(this, message.into_data()) {
      return Err(NetworkError::new(error, NetworkErrorKind::Protobuf));
    }

    Ok(())
  }
}
