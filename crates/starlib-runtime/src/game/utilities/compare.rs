use std::fmt::Debug;
use std::fmt::Display;

pub trait Cmp<T = Self>: Debug + Display + Send + Sync + 'static {
  fn cmp(&self, other: &T) -> bool;
}

macro_rules! implement {
  ($name:ident::$func:ident) => {
    #[derive(Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
    #[repr(transparent)]
    pub struct $name(pub u32);

    impl Cmp<u32> for $name {
      #[inline]
      fn cmp(&self, other: &u32) -> bool {
        other.$func(&self.0)
      }
    }

    impl ::core::fmt::Debug for $name {
      fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        write!(f, concat!(stringify!($name), "({})"), self.0)
      }
    }

    impl ::core::fmt::Display for $name {
      fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        write!(f, concat!(stringify!($name), "({})"), self.0)
      }
    }
  };
}

implement!(Eq::eq);
implement!(Ne::ne);

implement!(Lt::lt);
implement!(Le::le);

implement!(Gt::gt);
implement!(Ge::ge);
