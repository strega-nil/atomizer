#![cfg_attr(feature = "nightly", feature(marker_trait_attr))]
#![cfg_attr(feature = "nightly", feature(hash_raw_entry))]

pub mod stable_set;
//pub mod stable_map;

use std::{borrow::Borrow, hash::Hash, ops::Deref};

mod impls;

pub struct Atom<'a, T: ?Sized> {
  __ptr: &'a T,
}

impl<'a, T: ?Sized> Atom<'a, T> {
  fn new(__ptr: &'a T) -> Self {
    Atom { __ptr }
  }

  pub fn as_ref(p: Self) -> &'a T {
    p.__ptr
  }
}

/**
  This trait guarantees that, for a type T: StablePointer,
  given a value `t: &'a T`:

    `&**t` shall live for at least `'a`

  i.e., when `*t` moves, the underlying `<T as Deref>::Target` does not move
*/
#[cfg_attr(feature = "nightly", marker)]
pub unsafe trait StablePointer: Deref {}

unsafe impl<T: ?Sized> StablePointer for Box<T> {}
unsafe impl<T: ?Sized> StablePointer for std::rc::Rc<T> {}
unsafe impl<T: ?Sized> StablePointer for std::sync::Arc<T> {}
unsafe impl<T> StablePointer for Vec<T> {}
unsafe impl StablePointer for String {}

pub trait AtomProxy<Owned>
where
  Owned: Borrow<<Self as AtomProxy<Owned>>::Compare>,
{
  type Compare: ?Sized + Hash + Eq;

  fn to_owned(&self) -> Owned;
  fn to_compare(&self) -> &Self::Compare;
}

impl<T> AtomProxy<T> for <T as Deref>::Target
where
  T: Deref + Borrow<<T as Deref>::Target>,
  <T as Deref>::Target: Hash + Eq + ToOwned<Owned = T>,
{
  type Compare = <T as Deref>::Target;

  fn to_owned(&self) -> T {
    <Self::Compare as ToOwned>::to_owned(self)
  }
  fn to_compare(&self) -> &Self {
    self
  }
}
