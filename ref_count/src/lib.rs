use std::borrow::Borrow;
use std::cell::Cell;
use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ptr::NonNull;

pub struct RefCount<T> {
    control: NonNull<Control<T>>,
}

impl<T> RefCount<T> {
    pub fn new(value: T) -> Self {
        let control = move_to_heap(Control {
            value,
            count: Cell::new(1),
        });
        RefCount { control }
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
        this.control().count.get()
    }

    pub fn get_mut(this: &mut Self) -> Option<&mut T> {
        todo!()
    }

    pub fn ptr_eq(this: &Self, other: &Self) -> bool {
        todo!()
    }

    fn control(&self) -> &Control<T> {
        unsafe { self.control.as_ref() }
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
        self.control().update_count(|count| count + 1);
        RefCount {
            control: self.control,
        }
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
        &self.control().value
    }
}

impl<T> Drop for RefCount<T> {
    fn drop(&mut self) {
        let count = self.control().update_count(|count| count - 1);
        if count == 0 {
            drop_from_heap(self.control);
        }
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

struct Control<T> {
    count: Cell<usize>,
    value: T,
}

impl<T> Control<T> {
    fn update_count<F>(&self, f: F) -> usize
    where
        F: FnOnce(usize) -> usize,
    {
        let count = self.count.get();
        let count = f(count);
        self.count.set(count);
        count
    }
}

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

fn move_to_heap<T>(value: T) -> NonNull<T> {
    let b = Box::new(value);
    let ptr = Box::into_raw(b);
    NonNull::new(ptr).unwrap()
}

fn drop_from_heap<T>(ptr: NonNull<T>) {
    let ptr = ptr.as_ptr();
    let b = unsafe { Box::from_raw(ptr) };
    drop(b);
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;

    struct Inner<T, F>
    where
        F: FnMut(),
    {
        value: T,
        on_drop: F,
    }

    impl<T, F> Drop for Inner<T, F>
    where
        F: FnMut(),
    {
        fn drop(&mut self) {
            (self.on_drop)();
        }
    }

    #[test]
    fn test_basic() {
        let dropped = RefCell::new(false);

        let inner = Inner {
            value: 7,
            on_drop: || {
                *dropped.borrow_mut() = true;
            },
        };

        let rc1 = RefCount::new(inner);
        assert_eq!(RefCount::strong_count(&rc1), 1);
        assert_eq!(rc1.value, 7);

        let rc2 = RefCount::clone(&rc1);
        assert_eq!(RefCount::strong_count(&rc1), 2);
        assert_eq!(RefCount::strong_count(&rc2), 2);
        assert_eq!(rc1.value, 7);
        assert_eq!(rc2.value, 7);

        drop(rc1);
        assert_eq!(RefCount::strong_count(&rc2), 1);
        assert_eq!(rc2.value, 7);
        assert!(!*dropped.borrow());

        drop(rc2);
        assert!(*dropped.borrow());
    }
}
