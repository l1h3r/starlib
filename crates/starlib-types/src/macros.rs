macro_rules! define_bridge_enum {
  (
    #[bridge($proto:ident $(,NULL = $null:ident)?)]
    pub enum $name:ident {
      $($variant:ident),+
      $(,)?
    }
  ) => {
    #[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    #[repr(i32)]
    pub enum $name {
      $($variant = ::starlib_proto::$proto::$variant as i32),*
    }

    impl $name {
      // SAFETY:
      // - All protobuf enums are `#[repr(i32)]`
      // - All protobuf enums have an explicit `0` discriminant
      const ZERO: Self = Self::from_proto(unsafe { ::core::mem::transmute(0_i32) });

      #[inline]
      pub const fn as_str(&self) -> &'static str {
        match self {
          $(Self::$variant => stringify!($variant)),*
        }
      }

      #[inline]
      const fn from_proto(proto: ::starlib_proto::$proto) -> $name {
        match proto {
          $(::starlib_proto::$proto::NULL => $name::$null,)?
          $(::starlib_proto::$proto::$variant => $name::$variant),*
        }
      }

      #[inline]
      const fn into_proto(self) -> ::starlib_proto::$proto {
        match self {
          $($name::$variant => ::starlib_proto::$proto::$variant),*
        }
      }

      #[inline]
      const fn into_i32(self) -> i32 {
        self as i32
      }

      #[inline]
      const fn from_i32(this: i32) -> Self {
        $(
          if this == ::starlib_proto::$proto::$variant as i32 {
            return Self::$variant;
          }
        )*

        Self::ZERO
      }
    }

    impl ::core::default::Default for $name {
      #[inline]
      fn default() -> Self {
        Self::ZERO
      }
    }

    impl ::core::fmt::Debug for $name {
      fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.write_str(self.as_str())
      }
    }

    impl ::core::fmt::Display for $name {
      fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.write_str(self.as_str())
      }
    }

    impl $crate::traits::FromProto<::starlib_proto::$proto> for $name {
      #[inline]
      fn from_proto(other: ::starlib_proto::$proto) -> Self {
        $name::from_proto(other)
      }
    }

    impl $crate::traits::FromSc2<$name> for ::starlib_proto::$proto {
      #[inline]
      fn from_sc2(other: $name) -> Self {
        other.into_proto()
      }
    }

    impl $crate::traits::FromProto<i32> for $name {
      #[inline]
      fn from_proto(other: i32) -> Self {
        $name::from_i32(other)
      }
    }

    impl $crate::traits::FromSc2<$name> for i32 {
      #[inline]
      fn from_sc2(other: $name) -> Self {
        other.into_i32()
      }
    }
  };
}

#[rustfmt::skip]
macro_rules! field_type {
  (Self) => { Option<Box<Self>> };

  ($base:ty, vec) => { Vec<field_type!($base)> };
  ($base:ty, ref) => { field_type!($base) };
  ($base:ty, opt) => { Option<field_type!($base)> };
  ($base:ty) => { $base };
}

#[rustfmt::skip]
macro_rules! return_type {
  (String) => { &str };
  (Bytes) => { &[u8] };
  (Self) => { Option<&Self> };

  ($base:ty, vec) => { &[$base] };
  ($base:ty, ref) => { &$base };
  ($base:ty, opt) => { Option<&$base> };
  ($base:ty) => { $base };
}

#[rustfmt::skip]
macro_rules! return_value {
  ($self:ident, $name:ident, String) => { $self.$name.as_str() };
  ($self:ident, $name:ident, Bytes) => { &*$self.$name };
  ($self:ident, $name:ident, Self) => { $self.$name.as_deref() };

  ($self:ident, $name:ident, $_base:ty, vec) => { $self.$name.as_slice() };
  ($self:ident, $name:ident, $_base:ty, ref) => { &$self.$name };
  ($self:ident, $name:ident, $_base:ty, opt) => { $self.$name.as_ref() };
  ($self:ident, $name:ident, $_base:ty) => { $self.$name };
}

macro_rules! define_bridge_message {
  (
    #[bridge($proto:ident)]
    pub struct $name:ident {
      $($fname:ident: $ftype:ident $($ref:ident)? = proto::$pname:ident),*
      $(,)?
    }
  ) => {
    #[derive(Debug)]
    pub struct $name {
      $(pub(crate) $fname: field_type!($ftype $(, $ref)?)),*
    }

    impl $name {
      $(
        #[inline]
        pub fn $fname(&self) -> return_type!($ftype $(, $ref)?) {
          return_value!(self, $fname, $ftype $(, $ref)?)
        }
      )*
    }

    impl $crate::traits::FromProto<::starlib_proto::$proto> for $name {
      #[allow(unused_variables, reason = "macro-generated code")]
      #[inline]
      fn from_proto(other: ::starlib_proto::$proto) -> Self {
        Self {
          $(
            $fname: $crate::traits::FromProto::from_proto_internal(other.$pname)
          ),*
        }
      }
    }

    impl $crate::traits::FromSc2<$name> for ::starlib_proto::$proto {
      #[allow(unused_variables, reason = "macro-generated code")]
      #[inline]
      fn from_sc2(other: $name) -> Self {
        ::starlib_proto::$proto {
          $($pname: $crate::traits::IntoProto::into_proto(other.$fname)),*
        }
      }
    }
  };
}

macro_rules! define_bridge_oneof {
  (
    $(#[derive($($derive:tt)*)])?
    #[bridge($proto:ident)]
    pub enum $name:ident {
      None,
      $($variant:ident($discriminant:ty) = proto::$pname:ident),+
      $(,)?
    }
  ) => {
    $(#[derive($($derive)*)])?
    pub enum $name {
      None,
      $($variant($discriminant)),+
    }

    impl ::core::fmt::Debug for $name {
      fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        match self {
          Self::None => f.write_str("None"),
          $(Self::$variant(inner) => ::core::fmt::Debug::fmt(inner, f)),+
        }
      }
    }

    impl $crate::traits::FromProto<Option<::starlib_proto::$proto>> for $name {
      #[inline]
      fn from_proto(other: Option<::starlib_proto::$proto>) -> Self {
        let Some(data) = other else {
          return Self::None;
        };

        match data {
          $(
            ::starlib_proto::$proto::$pname(inner) => {
              Self::$variant($crate::traits::FromProto::from_proto(inner))
            }
          )+
        }
      }
    }

    impl $crate::traits::FromSc2<$name> for Option<::starlib_proto::$proto> {
      #[inline]
      fn from_sc2(other: $name) -> Self {
        match other {
          $name::None => {
            None
          }
          $(
            $name::$variant(inner) => {
              Some(::starlib_proto::$proto::$pname($crate::traits::FromSc2::from_sc2(inner)))
            }
          )+
        }
      }
    }
  };
}

macro_rules! binop_ref {
  ($name:ident<T: $bound:path>, $trait:ident::$method:ident, $assign_trait:ident::$assign_method:ident) => {
    binop_ref!(@impl<T: $bound> => $name<T>, $name<T>, $trait::$method, $assign_trait::$assign_method);
  };
  ($name:ident, $trait:ident::$method:ident, $assign_trait:ident::$assign_method:ident) => {
    binop_ref!($name, $name, $trait::$method, $assign_trait::$assign_method);
  };
  ($rhs:ident<T: $bound:path>, $lhs:ty, $trait:ident::$method:ident, $assign_trait:ident::$assign_method:ident) => {
    binop_ref!(@impl<T: $bound> => $rhs<T>, $lhs, $trait::$method, $assign_trait::$assign_method);
  };
  ($lhs:ty, $rhs:ty, $trait:ident::$method:ident, $assign_trait:ident::$assign_method:ident) => {
    binop_ref!(@impl => $lhs, $rhs, $trait::$method, $assign_trait::$assign_method);
  };
  (@impl $(<T: $bound:path>)? => $lhs:ty, $rhs:ty, $trait:ident::$method:ident, $assign_trait:ident::$assign_method:ident) => {
    impl $(<T: $bound>)? ::core::ops::$trait<&$rhs> for $lhs {
      type Output = $lhs;

      #[inline]
      fn $method(self, other: &$rhs) -> Self::Output {
        <$lhs>::$method(self, *other)
      }
    }

    impl $(<T: $bound>)? ::core::ops::$trait<$rhs> for &$lhs {
      type Output = $lhs;

      #[inline]
      fn $method(self, other: $rhs) -> Self::Output {
        <$lhs>::$method(*self, other)
      }
    }

    impl $(<T: $bound>)? ::core::ops::$trait<&$rhs> for &$lhs {
      type Output = $lhs;

      #[inline]
      fn $method(self, other: &$rhs) -> Self::Output {
        <$lhs>::$method(*self, *other)
      }
    }

    impl $(<T: $bound>)? ::core::ops::$assign_trait<$rhs> for $lhs {
      #[inline]
      fn $assign_method(&mut self, other: $rhs) {
        *self = ::core::ops::$trait::$method(*self, other);
      }
    }

    impl $(<T: $bound>)? ::core::ops::$assign_trait<&$rhs> for $lhs {
      #[inline]
      fn $assign_method(&mut self, other: &$rhs) {
        *self = ::core::ops::$trait::$method(*self, *other);
      }
    }
  };
}
