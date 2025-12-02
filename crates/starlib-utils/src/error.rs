use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

// -----------------------------------------------------------------------------
// Error Message
// -----------------------------------------------------------------------------

#[derive(Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct ErrorMessage(String);

impl ErrorMessage {
  #[inline]
  pub fn new<T>(message: T) -> Self
  where
    T: Display,
  {
    Self(message.to_string())
  }
}

impl Debug for ErrorMessage {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    Debug::fmt(&self.0, f)
  }
}

impl Display for ErrorMessage {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    Display::fmt(&self.0, f)
  }
}

impl Error for ErrorMessage {}

// -----------------------------------------------------------------------------
// Error Macro
// -----------------------------------------------------------------------------

#[macro_export]
macro_rules! define_error {
  (
    #[kind($error_kind_name:ident)]
    pub enum $error_name:ident {
      $($error_kind_variant:ident),+
      $(,)?
    }
  ) => {
    #[derive(Clone, Copy, Debug)]
    pub enum $error_kind_name {
      Message,
      $($error_kind_variant),+
    }

    pub struct $error_name {
      data: Box<dyn ::std::error::Error + 'static>,
      kind: $error_kind_name,
    }

    impl $error_name {
      #[inline]
      pub fn new<T>(data: T, kind: $error_kind_name) -> Self
      where
        T: ::std::error::Error + 'static,
      {
        Self {
          data: Box::new(data),
          kind,
        }
      }

      #[inline]
      pub fn msg<T>(message: T) -> Self
      where
        T: ::std::fmt::Display,
      {
        Self {
          data: Box::new($crate::error::ErrorMessage::new(message)),
          kind: $error_kind_name::Message,
        }
      }

      #[inline]
      pub fn kind(&self) -> $error_kind_name {
        self.kind
      }

      #[inline]
      pub fn downcast_source<T>(self) -> Result<T, Self>
      where
        T: ::std::error::Error + 'static,
      {
        match self.data.downcast() {
          Ok(this) => Ok(*this),
          Err(data) => Err(Self {
            data,
            kind: self.kind,
          }),
        }
      }
    }

    impl ::std::fmt::Debug for $error_name {
      #[inline]
      fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Debug::fmt(&self.data, f)
      }
    }

    impl ::std::fmt::Display for $error_name {
      #[inline]
      fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::std::fmt::Display::fmt(&self.data, f)
      }
    }

    impl ::std::error::Error for $error_name {
      #[inline]
      fn source(&self) -> Option<&(dyn ::std::error::Error + 'static)> {
        Some(&*self.data)
      }
    }
  };
}
