use core::borrow::Borrow;
use core::cmp;
use core::default::Default;
use core::hash;
use core::marker::PhantomData;

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

pub struct Borrowing<B: ?Sized, C>(C, PhantomData<fn(B)>);

impl<B: ?Sized, C> Borrowing<B, C> {
  pub fn new(c: C) -> Self {
    Self(c, PhantomData)
  }
}

impl<B: ?Sized, C: Default> Default for Borrowing<B, C> {
  fn default() -> Self {
    Self(C::default(), PhantomData)
  }
}

impl<B: ?Sized, C, T: ?Sized> Hash<T> for Borrowing<B, C>
where
  C: Hash<B>,
  T: Borrow<B>,
{
  fn hash<H: hash::Hasher>(&self, val: &T, state: &mut H) {
    self.0.hash(val.borrow(), state)
  }
}

impl<B: ?Sized, C, L: ?Sized, R: ?Sized> Eq<L, R> for Borrowing<B, C>
where
  C: Eq<B, B>,
  L: Borrow<B>,
  R: Borrow<B>,
{
  fn eq(&self, l: &L, r: &R) -> bool {
    self.0.eq(l.borrow(), r.borrow())
  }
}

pub struct Referencing<B: ?Sized, C = Natural<B>>(C, PhantomData<fn(&B)>);

impl<B: ?Sized, C> Referencing<B, C> {
  pub fn new(c: C) -> Self {
    Self(c, PhantomData)
  }
}

impl<B: ?Sized, C: Default> Default for Referencing<B, C> {
  fn default() -> Self {
    Self(C::default(), PhantomData)
  }
}

impl<B: ?Sized, C, T: ?Sized> Hash<T> for Referencing<B, C>
where
  C: Hash<B>,
  T: AsRef<B>,
{
  fn hash<H: hash::Hasher>(&self, val: &T, state: &mut H) {
    self.0.hash(val.as_ref(), state)
  }
}

impl<B: ?Sized, C, L: ?Sized, R: ?Sized> Eq<L, R> for Referencing<B, C>
where
  C: Eq<B, B>,
  L: AsRef<B>,
  R: AsRef<B>,
{
  fn eq(&self, l: &L, r: &R) -> bool {
    self.0.eq(l.as_ref(), r.as_ref())
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  pub fn referenced_u8_equality() {
    let v: Vec<u8> = vec![0x20, 0x61];
    let s = " a".to_string();
    let cmp: Referencing<[u8]> = Default::default();

    assert!(cmp.eq(&v, &s));
  }
}
