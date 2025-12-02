use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::str::FromStr;

use crate::version::Version;

// -----------------------------------------------------------------------------
// Parse Error Kind
// -----------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
pub enum ParseErrorKind {
  InvalidZero,
  InvalidData(&'static str),
  UnexpectedEnd,
  UnexpectedChar(char),
}

impl Display for ParseErrorKind {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Self::InvalidZero => {
        write!(f, "invalid leading zero")
      }
      Self::InvalidData(name) => {
        write!(f, "value of {name} version number exceeds u8::MAX")
      }
      Self::UnexpectedEnd => {
        write!(f, "unexpected end of input")
      }
      Self::UnexpectedChar(ch) => {
        write!(f, "unexpected character {ch}")
      }
    }
  }
}

// -----------------------------------------------------------------------------
// Parse Error
// -----------------------------------------------------------------------------

#[derive(Debug)]
pub struct ParseError {
  kind: ParseErrorKind,
}

impl ParseError {
  #[inline]
  const fn new(kind: ParseErrorKind) -> Self {
    Self { kind }
  }

  #[inline]
  pub const fn kind(&self) -> ParseErrorKind {
    self.kind
  }
}

impl Display for ParseError {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    Display::fmt(&self.kind, f)
  }
}

impl Error for ParseError {}

// -----------------------------------------------------------------------------
// Parse Version
// -----------------------------------------------------------------------------

impl FromStr for Version {
  type Err = ParseError;

  fn from_str(input: &str) -> Result<Self, Self::Err> {
    if input.is_empty() {
      return Err(ParseError::new(ParseErrorKind::UnexpectedEnd));
    }

    let (major, input): (u8, &str) = parse_number(input, "major")?;
    let input: &str = parse_divide(input)?;

    let (minor, input): (u8, &str) = parse_number(input, "minor")?;
    let input: &str = parse_divide(input)?;

    match parse_number(input, "patch") {
      Ok((patch, input)) => {
        if let Some(ch) = input.chars().next() {
          return Err(ParseError::new(ParseErrorKind::UnexpectedChar(ch)));
        }

        Ok(Self::new(major, minor, patch))
      }
      Err(error) => {
        if matches!(error.kind(), ParseErrorKind::UnexpectedEnd) {
          Ok(Self::new(major, minor, 0))
        } else {
          Err(error)
        }
      }
    }
  }
}

fn parse_number<'a>(input: &'a str, field: &'static str) -> Result<(u8, &'a str), ParseError> {
  let mut index: usize = 0;
  let mut value: u8 = 0;

  let bytes: &[u8] = input.as_bytes();

  while index < bytes.len() {
    let digit @ b'0'..=b'9' = bytes[index] else {
      break;
    };

    if digit == 0 && index > 0 {
      return Err(ParseError::new(ParseErrorKind::InvalidZero));
    }

    value = value
      .checked_mul(10)
      .and_then(|value| value.checked_add((digit - b'0') as u8))
      .ok_or_else(|| ParseError::new(ParseErrorKind::InvalidData(field)))?;

    index += 1;
  }

  if index > 0 {
    Ok((value, &input[..index]))
  } else if let Some(ch) = input[index..].chars().next() {
    Err(ParseError::new(ParseErrorKind::UnexpectedChar(ch)))
  } else {
    Err(ParseError::new(ParseErrorKind::UnexpectedEnd))
  }
}

fn parse_divide(input: &str) -> Result<&str, ParseError> {
  if let Some(input) = input.strip_prefix('.') {
    Ok(input)
  } else if let Some(ch) = input.chars().next() {
    Err(ParseError::new(ParseErrorKind::UnexpectedChar(ch)))
  } else {
    Err(ParseError::new(ParseErrorKind::UnexpectedEnd))
  }
}
