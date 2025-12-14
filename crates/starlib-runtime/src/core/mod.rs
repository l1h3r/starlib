#[macro_use]
mod macros;

mod operation;
mod utilities;

pub use self::operation::ControlFlow;
pub use self::operation::DynOperation;
pub use self::operation::IntoOperation;
pub use self::operation::Operation;
pub use self::utilities::Par;
pub use self::utilities::Seq;
pub use self::utilities::par;
pub use self::utilities::seq;
