use std::{borrow::Borrow, hash::Hash, ops::Deref};
use std::{cell, collections};

use super::{Atom, AtomProxy, StablePointer};

// note: we use a `HashMap` on nightly in order to get access to the
// `raw_entry` API
#[cfg(feature = "nightly")]
type HashSet<T> = collections::HashMap<T, ()>;

#[cfg(not(feature = "nightly"))]
type HashSet<T> = collections::HashSet<T>;

pub struct StableSet<T> {
  // for safety, this must be append-only
  // _never_ remove any values from it
  set: cell::UnsafeCell<HashSet<T>>,
}

/*
  Since we hold no pointers into the set
  (only into the elements themselves),
  we can send it across thread boundaries,
  as long as the members are Send, and <T as Deref>::Target: Sync
  (Send because we own them, Sync because we pass out pointers to them)
*/
unsafe impl<T> Send for StableSet<T>
where
  T: Send + Deref,
  <T as Deref>::Target: Sync,
{
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
  ) -> Atom<'a, <T as Deref>::Target>
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
