use crate::wrapper::{Borrowing, Referencing};
use core::borrow::Borrow;
use core::default::Default;
use core::marker::PhantomData;

pub trait Normalize<From, To> {
  fn normalize(&self, from: From) -> To;
}

impl<From, To, F: Fn(From) -> To> Normalize<From, To> for F {
  fn normalize(&self, from: From) -> To {
    self(from)
  }
}

pub struct Into<To>(PhantomData<fn() -> To>);

impl<To> Into<To> {
  pub fn new() -> Self {
    Self(PhantomData)
  }
}

impl<To> Default for Into<To> {
  fn default() -> Self {
    Self::new()
  }
}

impl<From, To> Normalize<From, To> for Into<To>
where
  To: core::convert::From<From>,
{
  fn normalize(&self, from: From) -> To {
    from.into()
  }
}

impl<'a, Ref: ?Sized + 'a, Wrapped, From: ?Sized, To> Normalize<&'a From, To>
  for Borrowing<Ref, Wrapped>
where
  From: Borrow<Ref>,
  Wrapped: Normalize<&'a Ref, To>,
{
  fn normalize(&self, from: &'a From) -> To {
    self.get_wrapped().normalize(from.borrow())
  }
}

impl<'a, Ref: ?Sized + 'a, Wrapped, From: ?Sized, To> Normalize<&'a From, To>
  for Referencing<Ref, Wrapped>
where
  From: AsRef<Ref>,
  Wrapped: Normalize<&'a Ref, To>,
{
  fn normalize(&self, from: &'a From) -> To {
    self.get_wrapped().normalize(from.as_ref())
  }
}
