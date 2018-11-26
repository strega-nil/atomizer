#![cfg_attr(feature = "nightly", feature(marker_trait_attr))]

pub mod stable_set;
//pub mod stable_map;

use std::{ops::Deref, borrow::Borrow};

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
pub unsafe trait StablePointer: Deref + Borrow<<Self as Deref>::Target> {}

unsafe impl<T: ?Sized> StablePointer for Box<T> {}
unsafe impl<T: ?Sized> StablePointer for std::rc::Rc<T> {}
unsafe impl<T: ?Sized> StablePointer for std::sync::Arc<T> {}
unsafe impl<T> StablePointer for Vec<T> {}
unsafe impl StablePointer for String {}
