use ::starlib_proto as proto;

use crate::core::Bytes;
use crate::core::Float;
use crate::core::Percent;
use crate::core::PlayerTag;
use crate::core::Point3DF;
use crate::core::PointF;
use crate::core::PointI;
use crate::core::RectI;
use crate::core::SizeI;
use crate::core::UnitTag;

// -----------------------------------------------------------------------------
// From Proto
// -----------------------------------------------------------------------------

pub trait FromProto<T: Default>: Sized {
  fn from_proto(other: T) -> Self;

  #[doc(hidden)]
  #[inline]
  fn from_proto_internal<I>(other: I) -> Self
  where
    I: Into<Option<T>>,
  {
    Self::from_proto(other.into().unwrap_or_default())
  }
}

// -----------------------------------------------------------------------------
// Into Sc2
// -----------------------------------------------------------------------------

pub trait IntoSc2<T>: Sized {
  fn into_sc2(self) -> T;
}

impl<T, U> IntoSc2<U> for T
where
  T: Default,
  U: FromProto<T>,
{
  #[inline]
  fn into_sc2(self) -> U {
    U::from_proto(self)
  }
}

// -----------------------------------------------------------------------------
// From Sc2
// -----------------------------------------------------------------------------

pub trait FromSc2<T>: Sized {
  fn from_sc2(other: T) -> Self;
}

// -----------------------------------------------------------------------------
// Into Proto
// -----------------------------------------------------------------------------

pub trait IntoProto<T>: Sized {
  fn into_proto(self) -> T;
}

impl<T, U> IntoProto<U> for T
where
  U: FromSc2<T>,
{
  #[inline]
  fn into_proto(self) -> U {
    U::from_sc2(self)
  }
}

// -----------------------------------------------------------------------------
// Implementations - Core Types (rust)
// -----------------------------------------------------------------------------

macro_rules! transparent {
  ($($type:ty)+) => {
    $(
      impl FromProto<$type> for $type {
        #[inline]
        fn from_proto(other: $type) -> Self {
          other
        }
      }

      impl FromSc2<$type> for $type {
        #[inline]
        fn from_sc2(other: $type) -> Self {
          other
        }
      }
    )+
  };
}

transparent!(f32 f64 i32 i64 u32 u64);
transparent!(bool String Vec<u8> ());

impl<T, U> FromProto<Box<U>> for Box<T>
where
  T: FromProto<U>,
  U: Default,
{
  #[inline]
  fn from_proto(other: Box<U>) -> Self {
    Box::new(T::from_proto(*other))
  }
}

impl<T, U> FromSc2<Box<U>> for Box<T>
where
  T: FromSc2<U>,
{
  #[inline]
  fn from_sc2(other: Box<U>) -> Self {
    Box::new(T::from_sc2(*other))
  }
}

impl<T, U> FromProto<Vec<U>> for Vec<T>
where
  T: FromProto<U>,
  U: Default,
{
  #[inline]
  fn from_proto(other: Vec<U>) -> Self {
    Self::from_iter(other.into_iter().map(T::from_proto))
  }
}

impl<T, U> FromSc2<Vec<U>> for Vec<T>
where
  T: FromSc2<U>,
{
  #[inline]
  fn from_sc2(other: Vec<U>) -> Self {
    Self::from_iter(other.into_iter().map(T::from_sc2))
  }
}

impl<T, U> FromProto<Option<U>> for Option<T>
where
  T: FromProto<U>,
  U: Default,
{
  #[inline]
  fn from_proto(other: Option<U>) -> Self {
    other.map(T::from_proto)
  }
}

impl<T, U> FromSc2<Option<Box<U>>> for Box<T>
where
  T: Default + FromSc2<U>,
{
  #[inline]
  fn from_sc2(other: Option<Box<U>>) -> Self {
    match other {
      Some(inner) => Box::new(T::from_sc2(*inner)),
      None => Box::new(T::default()),
    }
  }
}

impl<T, U> FromSc2<U> for Option<T>
where
  T: FromSc2<U>,
{
  #[inline]
  fn from_sc2(other: U) -> Self {
    Some(T::from_sc2(other))
  }
}

// -----------------------------------------------------------------------------
// Implementations - Core Types (crate)
// -----------------------------------------------------------------------------

transparent!(Bytes);

impl FromProto<f32> for Float {
  #[inline]
  fn from_proto(other: f32) -> Self {
    Self::from_f32(other)
  }
}

impl FromSc2<Float> for f32 {
  #[inline]
  fn from_sc2(other: Float) -> Self {
    other.into_f32()
  }
}

impl FromProto<f32> for Percent {
  #[inline]
  fn from_proto(other: f32) -> Self {
    Self::from_f32(other)
  }
}

impl FromSc2<Percent> for f32 {
  #[inline]
  fn from_sc2(other: Percent) -> Self {
    other.into_f32()
  }
}

impl FromProto<u32> for PlayerTag {
  #[inline]
  fn from_proto(other: u32) -> Self {
    Self::from_u32(other)
  }
}

impl FromSc2<PlayerTag> for u32 {
  #[inline]
  fn from_sc2(other: PlayerTag) -> Self {
    other.into_u32()
  }
}

impl FromProto<i32> for PlayerTag {
  #[inline]
  fn from_proto(other: i32) -> Self {
    Self::from_u32(other as u32)
  }
}

impl FromSc2<PlayerTag> for i32 {
  #[inline]
  fn from_sc2(other: PlayerTag) -> Self {
    other.into_u32() as i32
  }
}

impl FromProto<u64> for UnitTag {
  #[inline]
  fn from_proto(other: u64) -> Self {
    Self::from_u64(other)
  }
}

impl FromSc2<UnitTag> for u64 {
  #[inline]
  fn from_sc2(other: UnitTag) -> Self {
    other.into_u64()
  }
}

// -----------------------------------------------------------------------------
// Implementations - Core Types (geom)
// -----------------------------------------------------------------------------

impl FromProto<proto::Point2D> for PointF {
  #[inline]
  fn from_proto(other: proto::Point2D) -> Self {
    Self::new(other.x.into(), other.y.into())
  }
}

impl FromSc2<PointF> for proto::Point2D {
  #[inline]
  fn from_sc2(other: PointF) -> Self {
    proto::Point2D {
      x: other.x.into(),
      y: other.y.into(),
    }
  }
}

impl FromProto<proto::PointI> for PointI {
  #[inline]
  fn from_proto(other: proto::PointI) -> Self {
    Self::new(other.x, other.y)
  }
}

impl FromSc2<PointI> for proto::PointI {
  #[inline]
  fn from_sc2(other: PointI) -> Self {
    proto::PointI {
      x: other.x,
      y: other.y,
    }
  }
}

impl FromProto<proto::Point> for Point3DF {
  #[inline]
  fn from_proto(other: proto::Point) -> Self {
    Self::new(other.x.into(), other.y.into(), other.z.into())
  }
}

impl FromSc2<Point3DF> for proto::Point {
  #[inline]
  fn from_sc2(other: Point3DF) -> Self {
    proto::Point {
      x: other.x.into(),
      y: other.y.into(),
      z: other.z.into(),
    }
  }
}

impl FromProto<proto::Size2DI> for SizeI {
  #[inline]
  fn from_proto(other: proto::Size2DI) -> Self {
    Self::new(other.x, other.y)
  }
}

impl FromSc2<SizeI> for proto::Size2DI {
  #[inline]
  fn from_sc2(other: SizeI) -> Self {
    proto::Size2DI {
      x: other.w,
      y: other.h,
    }
  }
}

impl FromProto<proto::RectangleI> for RectI {
  #[inline]
  fn from_proto(other: proto::RectangleI) -> Self {
    let p0: proto::PointI = other.p0.unwrap_or_default();
    let p1: proto::PointI = other.p1.unwrap_or_default();

    Self {
      x: p0.x,
      y: p0.y,
      w: p1.x.strict_sub(p0.x),
      h: p1.y.strict_sub(p0.y),
    }
  }
}

impl FromSc2<RectI> for proto::RectangleI {
  #[inline]
  fn from_sc2(other: RectI) -> Self {
    proto::RectangleI {
      p0: Some(proto::PointI {
        x: other.x,
        y: other.y,
      }),
      p1: Some(proto::PointI {
        x: other.x.strict_add(other.w),
        y: other.y.strict_add(other.h),
      }),
    }
  }
}
