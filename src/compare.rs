use crate::wrapper::{Borrowing, Referencing};
use core::borrow::Borrow;
use core::default::Default;
use core::marker::PhantomData;
use core::{cmp, hash};

pub trait Hash<T: ?Sized> {
  fn hash<H: hash::Hasher>(&self, val: &T, state: &mut H);
}

pub trait Eq<L: ?Sized, R: ?Sized = L> {
  fn eq(&self, l: &L, r: &R) -> bool;
}

pub struct Natural<T: ?Sized>(PhantomData<fn(&T)>);

impl<T: ?Sized> Natural<T> {
  fn new() -> Self {
    Self(PhantomData)
  }
}

impl<T: ?Sized> Default for Natural<T> {
  fn default() -> Self {
    Self::new()
  }
}

impl<T: hash::Hash + ?Sized> Hash<T> for Natural<T> {
  fn hash<H: hash::Hasher>(&self, val: &T, state: &mut H) {
    val.hash(state);
  }
}

impl<T: cmp::Eq + ?Sized> Eq<T> for Natural<T> {
  fn eq(&self, l: &T, r: &T) -> bool {
    l == r
  }
}

impl<Ref: ?Sized, Wrapped, T: ?Sized> Hash<T> for Borrowing<Ref, Wrapped>
where
  Wrapped: Hash<Ref>,
  T: Borrow<Ref>,
{
  fn hash<H: hash::Hasher>(&self, val: &T, state: &mut H) {
    self.get_wrapped().hash(val.borrow(), state)
  }
}

impl<Ref: ?Sized, Wrapped, L: ?Sized, R: ?Sized> Eq<L, R>
  for Borrowing<Ref, Wrapped>
where
  Wrapped: Eq<Ref, Ref>,
  L: Borrow<Ref>,
  R: Borrow<Ref>,
{
  fn eq(&self, l: &L, r: &R) -> bool {
    self.get_wrapped().eq(l.borrow(), r.borrow())
  }
}

impl<Ref: ?Sized, Wrapped, T: ?Sized> Hash<T> for Referencing<Ref, Wrapped>
where
  Wrapped: Hash<Ref>,
  T: AsRef<Ref>,
{
  fn hash<H: hash::Hasher>(&self, val: &T, state: &mut H) {
    self.get_wrapped().hash(val.as_ref(), state)
  }
}

impl<Ref: ?Sized, Wrapped, L: ?Sized, R: ?Sized> Eq<L, R>
  for Referencing<Ref, Wrapped>
where
  Wrapped: Eq<Ref, Ref>,
  L: AsRef<Ref>,
  R: AsRef<Ref>,
{
  fn eq(&self, l: &L, r: &R) -> bool {
    self.get_wrapped().eq(l.as_ref(), r.as_ref())
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  pub fn referenced_u8_equality() {
    let v: Vec<u8> = vec![0x20, 0x61];
    let s = " a".to_string();
    let cmp: Referencing<[u8], Natural<[u8]>> = Default::default();

    assert!(cmp.eq(&v, &s));
  }
}
