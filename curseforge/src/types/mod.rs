use std::ops::Deref;

mod project;
mod query;

#[derive(Debug)]
#[repr(transparent)]
pub struct CurseResponse<T> {
  inner: T,
}

impl<T> Deref for CurseResponse<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.inner
  }
}

impl<T> CurseResponse<T> {
  pub fn new(inner: T) -> Self {
    Self { inner }
  }
}
