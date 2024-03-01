use std::{
    cell,
    ops::{Deref, DerefMut},
};

pub struct Ref<'a, T>(pub(crate) cell::Ref<'a, T>);

impl<'a, T> Deref for Ref<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

pub struct RefMut<'a, T>(pub(crate) cell::RefMut<'a, T>);

impl<'a, T> Deref for RefMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl<'a, T> DerefMut for RefMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.deref_mut()
    }
}
