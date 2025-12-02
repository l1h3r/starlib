//! StarLib Types

#[macro_use]
mod macros;

mod core;
mod exts;
mod game;
mod grid;

mod bridge;
mod stable;
mod traits;

pub use self::core::Bytes;
pub use self::core::BytesMut;
pub use self::core::Cost;
pub use self::core::Float;
pub use self::core::Path;
pub use self::core::PathBuf;
pub use self::core::Percent;
pub use self::core::PlayerTag;
pub use self::core::Point;
pub use self::core::Point3D;
pub use self::core::Point3DF;
pub use self::core::Point3DI;
pub use self::core::PointF;
pub use self::core::PointI;
pub use self::core::Rect;
pub use self::core::RectF;
pub use self::core::RectI;
pub use self::core::Size;
pub use self::core::SizeF;
pub use self::core::SizeI;
pub use self::core::Stat;
pub use self::core::UnitTag;

pub use self::game::Blip;
pub use self::game::GameData;
pub use self::game::GameInfo;
pub use self::game::GameState;
pub use self::game::UnitIter;
pub use self::game::UnitRef;
pub use self::game::Units;
pub use self::game::iter;

pub use self::grid::Pixel;
pub use self::grid::PixelGrid;
pub use self::grid::PrintGrid;
pub use self::grid::Scalar;
pub use self::grid::pixels;

pub use self::bridge::*;

pub use self::stable::AbilityId;
pub use self::stable::BuffId;
pub use self::stable::EffectId;
pub use self::stable::UnitTypeId;
pub use self::stable::UpgradeId;

pub use self::traits::ExtendProto;
pub use self::traits::FromProto;
pub use self::traits::FromSc2;
pub use self::traits::IntoProto;
pub use self::traits::IntoSc2;
pub use self::traits::Number;
