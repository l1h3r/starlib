// -----------------------------------------------------------------------------
// Bin Ops
// -----------------------------------------------------------------------------

macro_rules! op {
  (
    $name:ident[$($field:ident),+],
    $trait:ident::$method:ident($op:tt),
    $assign_trait:ident::$assign_method:ident
  ) => {
    impl<T> ::core::ops::$trait for $name<T>
    where
      T: $crate::traits::Number,
    {
      type Output = Self;

      #[inline]
      fn $method(self, other: Self) -> Self::Output {
        $name {
          $($field: self.$field $op other.$field),+
        }
      }
    }

    impl<T> ::core::ops::$trait<T> for $name<T>
    where
      T: $crate::traits::Number,
    {
      type Output = Self;

      #[inline]
      fn $method(self, other: T) -> Self::Output {
        $name {
          $($field: self.$field $op other),+
        }
      }
    }

    binop_ref!($name<T: $crate::traits::Number>, $trait::$method, $assign_trait::$assign_method);
    binop_ref!($name<T: $crate::traits::Number>, T, $trait::$method, $assign_trait::$assign_method);
  };
}
