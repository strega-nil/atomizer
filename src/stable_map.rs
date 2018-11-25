use std::borrow::Borrow;
use std::cmp::Ord;
use std::{cell, collections};

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

pub trait Atomizable: Ord + Borrow<<Self as Atomizable>::Comparable> {
  type Borrowed: ?Sized;
  type External: ?Sized;
  type Comparable: ?Sized + Ord;

  fn external_to_cmp(_: &Self::External) -> &Self::Comparable;
  fn as_borrowed(&self) -> &Self::Borrowed;
  fn from_external(_: &Self::External) -> Self;
}

pub struct AtomMap<T> {
  // for safety, this must be append-only
  // _never_ remove any values from it
  set: cell::UnsafeCell<collections::BTreeSet<T>>,
}

impl<T> AtomMap<T>
where
  T: Atomizable,
{
  pub fn new() -> Self {
    AtomMap {
      set: cell::UnsafeCell::new(collections::BTreeSet::new()),
    }
  }

  pub fn add_element<'a>(
    &'a self,
    element: &T::External,
  ) -> Atom<'a, T::Borrowed> {
    unsafe {
      // safe because we don't allow anybody to get a reference to the innards
      // without an indirection
      // and because we never remove
      let name_cmp = T::external_to_cmp(element);
      let inner = &mut *self.set.get();
      if let Some(b) = inner.get(name_cmp) {
        let buf = &*(b as *const T);
        Atom::new(buf.as_borrowed())
      } else {
        inner.insert(T::from_external(element));
        // this seems unnecessary, but BTreeSet doesn't have a full interface
        // also, break the lifetime relation between inner and the ref
        let buf = &*(inner.get(name_cmp).unwrap() as *const T);
        Atom::new(buf.as_borrowed())
      }
    }
  }
}
