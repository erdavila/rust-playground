use std::borrow::Borrow;
use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::Deref;

pub struct RefCount<T> {
    phantom: PhantomData<T>,
}

impl<T> RefCount<T> {
    pub fn new(value: T) -> Self {
        todo!()
    }

    pub fn try_unwrap(this: Self) -> Result<T, Self> {
        todo!()
    }

    pub fn into_raw(this: Self) -> *const T {
        todo!()
    }

    pub fn as_ptr(this: &Self) -> *const T {
        todo!()
    }

    /// # Safety
    pub unsafe fn from_raw(ptr: *const T) -> Self {
        todo!()
    }

    pub fn downgrade(this: &Self) -> WeakRef<T> {
        todo!()
    }

    pub fn weak_count(this: &Self) -> usize {
        todo!()
    }

    pub fn strong_count(this: &Self) -> usize {
        todo!()
    }

    pub fn get_mut(this: &mut Self) -> Option<&mut T> {
        todo!()
    }

    pub fn ptr_eq(this: &Self, other: &Self) -> bool {
        todo!()
    }
}

impl<T> AsRef<T> for RefCount<T> {
    fn as_ref(&self) -> &T {
        todo!()
    }
}

impl<T> Borrow<T> for RefCount<T> {
    fn borrow(&self) -> &T {
        todo!()
    }
}

impl<T> Clone for RefCount<T> {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl<T> Default for RefCount<T>
where
    T: Default,
{
    fn default() -> Self {
        todo!()
    }
}

impl<T> Deref for RefCount<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        todo!()
    }
}

impl<T> Drop for RefCount<T> {
    fn drop(&mut self) {
        todo!()
    }
}

impl<T> From<T> for RefCount<T> {
    fn from(value: T) -> Self {
        todo!()
    }
}

impl<T> Hash for RefCount<T>
where
    T: Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        todo!()
    }
}

impl<T> Ord for RefCount<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        todo!()
    }
}

impl<T> PartialEq for RefCount<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl<T> PartialOrd for RefCount<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        todo!()
    }
}

impl<T> Eq for RefCount<T> where T: Eq {}

pub struct WeakRef<T> {
    phantom: PhantomData<T>,
}

impl<T> WeakRef<T> {
    pub fn new() -> Self {
        todo!()
    }

    pub fn as_ptr(&self) -> *const T {
        todo!()
    }

    pub fn into_raw(self) -> *const T {
        todo!()
    }

    /// # Safety
    pub unsafe fn from_raw(ptr: *const T) -> Self {
        todo!()
    }

    pub fn upgrade(&self) -> Option<RefCount<T>> {
        todo!();
    }

    pub fn strong_count(&self) -> usize {
        todo!()
    }

    pub fn weak_count(&self) -> usize {
        todo!()
    }

    pub fn ptr_eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl<T> Clone for WeakRef<T> {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl<T> Default for WeakRef<T> {
    fn default() -> Self {
        todo!()
    }
}

impl<T> Drop for WeakRef<T> {
    fn drop(&mut self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_() {}
}
