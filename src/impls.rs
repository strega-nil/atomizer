use std::{fmt, hash};

use super::Atom;

impl<'a, T: ?Sized> Copy for Atom<'a, T> {}
impl<'a, T: ?Sized> Clone for Atom<'a, T> {
  fn clone(&self) -> Self {
    *self
  }
}

impl<'a, T: ?Sized + fmt::Debug> fmt::Debug for Atom<'a, T> {
  fn fmt(&self, x: &mut fmt::Formatter) -> fmt::Result {
    self.__ptr.fmt(x)
  }
}
impl<'a, T: ?Sized + fmt::Display> fmt::Display for Atom<'a, T> {
  fn fmt(&self, x: &mut fmt::Formatter) -> fmt::Result {
    self.__ptr.fmt(x)
  }
}

impl<'a, T: ?Sized> hash::Hash for Atom<'a, T> {
  fn hash<H: hash::Hasher>(&self, h: &mut H) {
    (self.__ptr as *const T).hash(h)
  }
}

impl<'a, T: 'a + ?Sized> std::ops::Deref for Atom<'a, T> {
  type Target = T;

  fn deref(&self) -> &T {
    self.__ptr
  }
}

impl<'a, T: 'a + ?Sized> PartialEq for Atom<'a, T> {
  fn eq(&self, other: &Self) -> bool {
    (self.__ptr as *const _) == (other.__ptr as *const _)
  }
}
impl<'a, T: 'a + ?Sized> Eq for Atom<'a, T> {}
