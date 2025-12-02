//! StarLib Network Interface

#[macro_use]
extern crate starlib_utils;

mod builder;
mod retries;
mod socket;
mod traits;

pub use self::builder::Builder;
pub use self::retries::Exponential;
pub use self::retries::Fibonacci;
pub use self::retries::Interval;
pub use self::socket::NetworkError;
pub use self::socket::NetworkErrorKind;
pub use self::socket::WebSocket;
pub use self::traits::Decode;
pub use self::traits::Encode;
