use std::{borrow, cmp, convert, fmt, hash, ops};

use super::Atom;

impl<'a, T: ?Sized> Copy for Atom<'a, T> {}
impl<'a, T: ?Sized> Clone for Atom<'a, T> {
  fn clone(&self) -> Self {
    *self
  }
}

// ORDERING AND HASHING

impl<'a, T: 'a + ?Sized> PartialEq for Atom<'a, T> {
  fn eq(&self, other: &Self) -> bool {
    (self.ptr as *const _) == (other.ptr as *const _)
  }
}
impl<'a, T: 'a + ?Sized> Eq for Atom<'a, T> {}

impl<'a, T: 'a + ?Sized + PartialOrd> PartialOrd for Atom<'a, T> {
  fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
    (**self).partial_cmp(&**other)
  }

  fn lt(&self, other: &Self) -> bool {
    (**self) < (**other)
  }
  fn gt(&self, other: &Self) -> bool {
    (**self) > (**other)
  }
  fn le(&self, other: &Self) -> bool {
    (**self) <= (**other)
  }
  fn ge(&self, other: &Self) -> bool {
    (**self) >= (**other)
  }
}
impl<'a, T: 'a + ?Sized + Ord> Ord for Atom<'a, T> {
  fn cmp(&self, other: &Self) -> cmp::Ordering {
    (**self).cmp(&**other)
  }
}

impl<'a, T: ?Sized> hash::Hash for Atom<'a, T> {
  fn hash<H: hash::Hasher>(&self, h: &mut H) {
    (self.ptr as *const T).hash(h)
  }
}

// CONVERSIONS

impl<'a, T: 'a + ?Sized> ops::Deref for Atom<'a, T> {
  type Target = T;

  fn deref(&self) -> &T {
    self.ptr
  }
}

impl<'a, T: ?Sized> borrow::Borrow<T> for Atom<'a, T> {
  fn borrow(&self) -> &T {
    self
  }
}

impl<'a, T: ?Sized> convert::AsRef<T> for Atom<'a, T> {
  fn as_ref(&self) -> &T {
    self
  }
}

// FORMATTING

impl<'a, T: ?Sized + fmt::Debug> fmt::Debug for Atom<'a, T> {
  fn fmt(&self, x: &mut fmt::Formatter) -> fmt::Result {
    <T as fmt::Debug>::fmt(self.ptr, x)
  }
}
impl<'a, T: ?Sized + fmt::Display> fmt::Display for Atom<'a, T> {
  fn fmt(&self, x: &mut fmt::Formatter) -> fmt::Result {
    <T as fmt::Display>::fmt(self.ptr, x)
  }
}

impl<'a, T: ?Sized + fmt::Binary> fmt::Binary for Atom<'a, T> {
  fn fmt(&self, x: &mut fmt::Formatter) -> fmt::Result {
    <T as fmt::Binary>::fmt(self.ptr, x)
  }
}
impl<'a, T: ?Sized + fmt::LowerExp> fmt::LowerExp for Atom<'a, T> {
  fn fmt(&self, x: &mut fmt::Formatter) -> fmt::Result {
    <T as fmt::LowerExp>::fmt(self.ptr, x)
  }
}
impl<'a, T: ?Sized + fmt::LowerHex> fmt::LowerHex for Atom<'a, T> {
  fn fmt(&self, x: &mut fmt::Formatter) -> fmt::Result {
    <T as fmt::LowerHex>::fmt(self.ptr, x)
  }
}
impl<'a, T: ?Sized + fmt::Octal> fmt::Octal for Atom<'a, T> {
  fn fmt(&self, x: &mut fmt::Formatter) -> fmt::Result {
    <T as fmt::Octal>::fmt(self.ptr, x)
  }
}
impl<'a, T: ?Sized> fmt::Pointer for Atom<'a, T> {
  fn fmt(&self, x: &mut fmt::Formatter) -> fmt::Result {
    <&'a T as fmt::Pointer>::fmt(&self.ptr, x)
  }
}
impl<'a, T: ?Sized + fmt::UpperExp> fmt::UpperExp for Atom<'a, T> {
  fn fmt(&self, x: &mut fmt::Formatter) -> fmt::Result {
    <T as fmt::UpperExp>::fmt(self.ptr, x)
  }
}
impl<'a, T: ?Sized + fmt::UpperHex> fmt::UpperHex for Atom<'a, T> {
  fn fmt(&self, x: &mut fmt::Formatter) -> fmt::Result {
    <T as fmt::UpperHex>::fmt(self.ptr, x)
  }
}
