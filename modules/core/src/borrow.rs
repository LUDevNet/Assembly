//! # Utilities for borrowing
use std::borrow::{Borrow, BorrowMut};

/// An enum that provides a mutable reference by either owning or
/// borrowing a struct (Own or Mutable)
pub enum Oom<'a, T> {
    Own(T),
    Mut(&'a mut T),
}

impl<'a, T> AsMut<T> for Oom<'a, T> {
    fn as_mut(&mut self) -> &mut T {
        match self {
            Oom::Own(data) => data,
            Oom::Mut(data) => data,
        }
    }
}

impl<'a, T> AsRef<T> for Oom<'a, T> {
    fn as_ref(&self) -> &T {
        match self {
            Oom::Own(data) => data,
            Oom::Mut(data) => data,
        }
    }
}

impl<'a, T> Borrow<T> for Oom<'a, T> {
    fn borrow(&self) -> &T {
        self.as_ref()
    }
}

impl<'a, T> BorrowMut<T> for Oom<'a, T> {
    fn borrow_mut(&mut self) -> &mut T {
        self.as_mut()
    }
}

impl<'a, T> From<T> for Oom<'a, T> {
    fn from(own: T) -> Self {
        Oom::Own(own)
    }
}

impl<'a, T> From<&'a mut T> for Oom<'a, T> {
    fn from(some: &'a mut T) -> Self {
        Oom::Mut(some)
    }
}
