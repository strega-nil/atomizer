use std::{borrow::Borrow, cmp::Ord};
use std::{cell, collections};

use super::{Atom, AtomProxy, StablePointer};

pub struct StableSet<T: StablePointer> {
  // for safety, this must be append-only
  // _never_ remove any values from it
  set: cell::UnsafeCell<collections::BTreeSet<T>>,
}

impl<T> StableSet<T>
where
  T: StablePointer + Ord,
{
  pub fn new() -> Self {
    StableSet {
      set: cell::UnsafeCell::new(collections::BTreeSet::new()),
    }
  }

  pub fn add_element<'a, 'b, U>(
    &'a self,
    element: &'b U,
  ) -> Atom<'a, <T as std::ops::Deref>::Target>
  where
    U: ?Sized + AtomProxy<T>,
    T: Borrow<<U as AtomProxy<T>>::Compare>,
  {
    unsafe {
      // safe because we don't allow anybody to get a reference to the innards
      // without an indirection
      // and because we never remove
      let name_cmp = element.to_compare();
      let inner = &mut *self.set.get();
      if let Some(b) = inner.get(name_cmp) {
        let buf = &*(b as *const T);
        Atom::new(buf.borrow())
      } else {
        inner.insert(element.to_owned());
        // this seems unnecessary, but BTreeSet doesn't have a full interface
        // also, break the lifetime relation between inner and the ref
        let buf = &*(inner.get(name_cmp).unwrap() as *const T);
        Atom::new(buf.borrow())
      }
    }
  }
}
