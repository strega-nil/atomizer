#![cfg_attr(feature = "nightly", feature(marker_trait_attr))]
#![cfg_attr(feature = "nightly", feature(hash_raw_entry))]
#![cfg_attr(feature = "nightly", feature(fundamental))]

pub mod stable_set;
//pub mod stable_map;

use std::{borrow::Borrow, hash::Hash, ops::Deref};

mod impls;

/**
  An `Atom<'a, T>` is almost equivalent to an `&'a T`,
  except that equality compares the pointer identity, rather than
  doing a deep equality comparison.
*/
#[cfg_attr(feature = "nightly", fundamental)]
pub struct Atom<'a, T: ?Sized> {
  ptr: &'a T,
}

impl<'a, T: ?Sized> Atom<'a, T> {
  fn new(ptr: &'a T) -> Self {
    Atom { ptr }
  }

  pub fn as_ref(p: Self) -> &'a T {
    p.ptr
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
unsafe impl StablePointer for std::ffi::CString {}
unsafe impl StablePointer for std::ffi::OsString {}

pub trait AtomProxy<Owned>
where
  Owned: Borrow<<Self as AtomProxy<Owned>>::Compare>,
{
  type Compare: ?Sized + Hash + Eq;

  fn to_owned(&self) -> Owned;
  fn to_compare(&self) -> &Self::Compare;
}

impl<T, Owned> AtomProxy<Owned> for T
where
  Owned: Borrow<T>,
  T: Hash + Eq,
  for <'a> &'a T: Into<Owned>
{
  type Compare = T;

  fn to_owned(&self) -> Owned {
    <&Self as Into<Owned>>::into(self)
  }
  fn to_compare(&self) -> &Self {
    self
  }
}
