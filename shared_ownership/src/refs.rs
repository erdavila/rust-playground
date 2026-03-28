use std::cell;
use std::ops::{Deref, DerefMut};

pub struct Ref<'a, T>(pub(crate) cell::Ref<'a, T>);

impl<T> Deref for Ref<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct RefMut<'a, T>(pub(crate) cell::RefMut<'a, T>);

impl<T> Deref for RefMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for RefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
