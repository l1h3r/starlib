#[macro_use]
mod macros;

mod angle;
mod iter;
mod point2d;
mod point3d;
mod rect;
mod rotation;
mod size;
mod transform;

pub use self::angle::Angle;
pub use self::iter::RectIter;
pub use self::point2d::Point;
pub use self::point3d::Point3D;
pub use self::rect::Rect;
pub use self::rotation::Rotation;
pub use self::size::Size;
pub use self::transform::Transform;

use crate::core::Float;

pub type AngleF = Angle<Float>;
pub type AngleI = Angle<i32>;

pub type Point3DF = Point3D<Float>;
pub type Point3DI = Point3D<i32>;

pub type PointF = Point<Float>;
pub type PointI = Point<i32>;

pub type RectF = Rect<Float>;
pub type RectI = Rect<i32>;

pub type RotationF = Rotation<Float>;
pub type RotationI = Rotation<i32>;

pub type SizeF = Size<Float>;
pub type SizeI = Size<i32>;

pub type TransformF = Transform<Float>;
pub type TransformI = Transform<i32>;
