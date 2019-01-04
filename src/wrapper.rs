use core::marker::PhantomData;

pub struct Borrowing<Ref: ?Sized, Wrapped>(Wrapped, PhantomData<fn(&Ref)>);

impl<Ref: ?Sized, Wrapped> Borrowing<Ref, Wrapped> {
  pub fn new(w: Wrapped) -> Self {
    Self(w, PhantomData)
  }

  pub fn get_wrapped(&self) -> &Wrapped {
    &self.0
  }
}

impl<Ref: ?Sized, Wrapped: Default> Default for Borrowing<Ref, Wrapped> {
  fn default() -> Self {
    Self(Default::default(), PhantomData)
  }
}

pub struct Referencing<Ref: ?Sized, Wrapped>(Wrapped, PhantomData<fn(&Ref)>);

impl<Ref: ?Sized, Wrapped> Referencing<Ref, Wrapped> {
  pub fn new(w: Wrapped) -> Self {
    Self(w, PhantomData)
  }

  pub fn get_wrapped(&self) -> &Wrapped {
    &self.0
  }
}

impl<Ref: ?Sized, Wrapped: Default> Default for Referencing<Ref, Wrapped> {
  fn default() -> Self {
    Self(Default::default(), PhantomData)
  }
}
