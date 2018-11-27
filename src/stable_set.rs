use std::{borrow::Borrow, hash::Hash};
use std::{cell, collections};

use super::{Atom, AtomProxy, StablePointer};

#[cfg(feature = "nightly")]
pub struct StableSet<T: StablePointer> {
  // for safety, this must be append-only
  // _never_ remove any values from it
  set: cell::UnsafeCell<collections::HashMap<T, ()>>,
}

#[cfg(not(feature = "nightly"))]
pub struct StableSet<T: StablePointer> {
  // for safety, this must be append-only
  // _never_ remove any values from it
  set: cell::UnsafeCell<collections::HashSet<T>>,
}

#[cfg(feature = "nightly")]
fn add_element_insert<'a, 'b, T, U>(
  set: &'a mut collections::HashMap<T, ()>,
  element: &U,
) -> *const T
where
  U: ?Sized + AtomProxy<T>,
  T: Borrow<<U as AtomProxy<T>>::Compare> + Hash + Eq,
{
  use std::collections::hash_map::RawEntryMut;
  let cmp = element.to_compare();
  let entry = set.raw_entry_mut().from_key(cmp);
  match entry {
    RawEntryMut::Occupied(entry) => entry.key(),
    RawEntryMut::Vacant(entry) => {
      let (key, _) = entry.insert(element.to_owned(), ());
      key
    }
  }
}

#[cfg(not(feature = "nightly"))]
fn add_element_insert<'a, 'b, T, U>(
  set: &'a mut collections::HashSet<T>,
  element: &U,
) -> *const T
where
  U: ?Sized + AtomProxy<T>,
  T: Borrow<<U as AtomProxy<T>>::Compare> + Hash + Eq,
{
  let cmp = element.to_compare();
  if let Some(b) = set.get(cmp) {
    b
  } else {
    set.insert(element.to_owned());
    // this seems unnecessary, but HashSet doesn't have a full interface
    // also, break the lifetime relation between inner and the ref
    set.get(cmp).unwrap()
  }
}

impl<T> StableSet<T>
where
  T: StablePointer + Hash + Eq,
{
  #[cfg(feature = "nightly")]
  pub fn new() -> Self {
    StableSet {
      set: cell::UnsafeCell::new(collections::HashMap::new()),
    }
  }

  #[cfg(not(feature = "nightly"))]
  pub fn new() -> Self {
    StableSet {
      set: cell::UnsafeCell::new(collections::HashSet::new()),
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
      let inner = &mut *self.set.get();
      Atom::new(&*add_element_insert(inner, element))
    }
  }
}
