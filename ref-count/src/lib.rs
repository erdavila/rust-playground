use std::borrow::Borrow;
use std::cell::Cell;
use std::hash::Hash;
use std::mem::{self, MaybeUninit, forget};
use std::ops::Deref;
use std::ptr::NonNull;

pub struct RefCount<T> {
    control: NonNull<Control<T>>,
}

impl<T> RefCount<T> {
    pub fn new(value: T) -> Self {
        let control = move_to_heap(Control {
            value: ValueHolder::new(value),
            strong_count: Count::new(1),
            weak_count: Count::new(0),
        });
        RefCount { control }
    }

    #[expect(clippy::missing_errors_doc)]
    pub fn try_unwrap(mut this: Self) -> Result<T, Self> {
        if Self::strong_count(&this) == 1 {
            let control = this.control_mut();
            control.strong_count.dec();
            let value = control.value.move_out();
            if control.weak_count.get() == 0 {
                drop_from_heap(this.control);
            }
            forget(this);
            Ok(value)
        } else {
            Err(this)
        }
    }

    #[must_use]
    pub fn into_raw(this: Self) -> *const T {
        let ptr = this.control().value.get_ptr();
        forget(this);
        ptr
    }

    #[must_use]
    pub fn as_ptr(this: &Self) -> *const T {
        this.control().value.get_ptr()
    }

    /// # Safety
    pub unsafe fn from_raw(ptr: *const T) -> Self {
        RefCount {
            control: unsafe { Control::ptr_from_raw(ptr) },
        }
    }

    #[must_use]
    pub fn downgrade(this: &Self) -> WeakRef<T> {
        this.control().weak_count.inc();
        WeakRef {
            control: this.control,
        }
    }

    #[must_use]
    pub fn weak_count(this: &Self) -> usize {
        this.control().weak_count.get()
    }

    #[must_use]
    pub fn strong_count(this: &Self) -> usize {
        this.control().strong_count.get()
    }

    pub fn get_mut(this: &mut Self) -> Option<&mut T> {
        if Self::strong_count(this) == 1 && Self::weak_count(this) == 0 {
            Some(this.control_mut().value.get_mut())
        } else {
            None
        }
    }

    #[must_use]
    pub fn ptr_eq(this: &Self, other: &Self) -> bool {
        this.control().value.get_ptr() == other.control().value.get_ptr()
    }

    fn control(&self) -> &Control<T> {
        unsafe { self.control.as_ref() }
    }

    fn control_mut(&mut self) -> &mut Control<T> {
        unsafe { self.control.as_mut() }
    }
}

impl<T> AsRef<T> for RefCount<T> {
    fn as_ref(&self) -> &T {
        self.control().value.get_ref()
    }
}

impl<T> Borrow<T> for RefCount<T> {
    fn borrow(&self) -> &T {
        self.control().value.get_ref()
    }
}

impl<T> Clone for RefCount<T> {
    fn clone(&self) -> Self {
        self.control().strong_count.inc();
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
        Self::new(Default::default())
    }
}

impl<T> Deref for RefCount<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.control().value.get_ref()
    }
}

impl<T> Drop for RefCount<T> {
    fn drop(&mut self) {
        let control = self.control_mut();
        let count = control.strong_count.dec();
        if count == 0 {
            control.value.drop_in_place();

            if Self::weak_count(self) == 0 {
                drop_from_heap(self.control);
            }
        }
    }
}

impl<T> From<T> for RefCount<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T> Hash for RefCount<T>
where
    T: Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_ref().hash(state);
    }
}

impl<T> Ord for RefCount<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_ref().cmp(other.as_ref())
    }
}

impl<T> PartialEq for RefCount<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl<T> PartialOrd for RefCount<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.as_ref().partial_cmp(other.as_ref())
    }
}

impl<T> Eq for RefCount<T> where T: Eq {}

struct Control<T> {
    strong_count: Count,
    weak_count: Count,
    value: ValueHolder<T>,
}

impl<T> Control<T> {
    unsafe fn ptr_from_raw(ptr: *const T) -> NonNull<Self> {
        let offset = mem::offset_of!(Self, value.value);
        let control_ptr = unsafe { ptr.cast::<u8>().sub(offset) as *mut Self };
        NonNull::new(control_ptr).unwrap()
    }
}

struct Count {
    cell: Cell<usize>,
}

impl Count {
    fn new(count: usize) -> Self {
        Count {
            cell: Cell::new(count),
        }
    }

    fn get(&self) -> usize {
        self.cell.get()
    }

    fn inc(&self) -> usize {
        self.update(|count| count + 1)
    }

    fn dec(&self) -> usize {
        self.update(|count| count - 1)
    }

    fn update<F>(&self, f: F) -> usize
    where
        F: FnOnce(usize) -> usize,
    {
        let count = self.cell.get();
        let count = f(count);
        self.cell.set(count);
        count
    }
}

pub struct ValueHolder<T> {
    value: MaybeUninit<T>,
    #[cfg(test)]
    initialized: bool,
}

impl<T> ValueHolder<T> {
    fn new(value: T) -> ValueHolder<T> {
        ValueHolder {
            value: MaybeUninit::new(value),
            #[cfg(test)]
            initialized: true,
        }
    }

    fn empty() -> ValueHolder<T> {
        ValueHolder {
            value: MaybeUninit::uninit(),
            #[cfg(test)]
            initialized: false,
        }
    }

    fn get_ref(&self) -> &T {
        self.assert_initialized();
        unsafe { self.value.assume_init_ref() }
    }

    fn get_mut(&mut self) -> &mut T {
        self.assert_initialized();
        unsafe { self.value.assume_init_mut() }
    }

    fn get_ptr(&self) -> *const T {
        self.value.as_ptr()
    }

    fn drop_in_place(&mut self) {
        self.assert_initialized();
        unsafe { self.value.assume_init_drop() };
        self.set_initialized(false);
    }

    fn move_out(&mut self) -> T {
        self.set_initialized(false);
        unsafe { self.value.assume_init_read() }
    }

    #[cfg(test)]
    fn assert_initialized(&self) {
        assert!(self.initialized);
    }
    #[cfg(not(test))]
    #[expect(clippy::unused_self)]
    fn assert_initialized(&self) {}

    #[cfg(test)]
    fn set_initialized(&mut self, value: bool) {
        self.initialized = value;
    }
    #[cfg(not(test))]
    #[expect(clippy::unused_self)]
    fn set_initialized(&mut self, _: bool) {}
}

pub struct WeakRef<T> {
    control: NonNull<Control<T>>,
}

impl<T> WeakRef<T> {
    #[must_use]
    pub fn new() -> Self {
        let control = move_to_heap(Control {
            value: ValueHolder::empty(),
            strong_count: Count::new(0),
            weak_count: Count::new(1),
        });
        WeakRef { control }
    }

    fn control(&self) -> &Control<T> {
        unsafe { self.control.as_ref() }
    }

    #[must_use]
    pub fn as_ptr(&self) -> *const T {
        self.control().value.get_ptr()
    }

    #[must_use]
    pub fn into_raw(self) -> *const T {
        let ptr = self.control().value.get_ptr();
        forget(self);
        ptr
    }

    #[expect(clippy::missing_safety_doc)]
    pub unsafe fn from_raw(ptr: *const T) -> Self {
        WeakRef {
            control: unsafe { Control::ptr_from_raw(ptr) },
        }
    }

    #[must_use]
    pub fn upgrade(&self) -> Option<RefCount<T>> {
        if self.strong_count() > 0 {
            self.control().strong_count.inc();
            Some(RefCount {
                control: self.control,
            })
        } else {
            None
        }
    }

    #[must_use]
    pub fn strong_count(&self) -> usize {
        self.control().strong_count.get()
    }

    #[must_use]
    pub fn weak_count(&self) -> usize {
        self.control().weak_count.get()
    }

    #[must_use]
    pub fn ptr_eq(&self, other: &Self) -> bool {
        self.control().value.get_ptr() == other.control().value.get_ptr()
    }
}

impl<T> Clone for WeakRef<T> {
    fn clone(&self) -> Self {
        self.control().weak_count.inc();
        WeakRef {
            control: self.control,
        }
    }
}

impl<T> Default for WeakRef<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for WeakRef<T> {
    fn drop(&mut self) {
        let count = self.control().weak_count.dec();
        if count == 0 && self.strong_count() == 0 {
            drop_from_heap(self.control);
        }
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
    use std::ptr;

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

    #[test]
    fn test_weak() {
        let dropped = RefCell::new(false);

        let inner = Inner {
            value: 7,
            on_drop: || {
                *dropped.borrow_mut() = true;
            },
        };

        let rc1 = RefCount::new(inner);
        assert_eq!(RefCount::strong_count(&rc1), 1);
        assert_eq!(RefCount::weak_count(&rc1), 0);

        let w1 = RefCount::downgrade(&rc1);
        assert_eq!(RefCount::strong_count(&rc1), 1);
        assert_eq!(w1.strong_count(), 1);
        assert_eq!(RefCount::weak_count(&rc1), 1);
        assert_eq!(w1.weak_count(), 1);

        let rc2 = w1.upgrade().expect("Upgraded ref should be Some(_)");
        assert_eq!(RefCount::strong_count(&rc1), 2);
        assert_eq!(RefCount::strong_count(&rc2), 2);
        assert_eq!(w1.strong_count(), 2);
        assert_eq!(RefCount::weak_count(&rc1), 1);
        assert_eq!(RefCount::weak_count(&rc2), 1);
        assert_eq!(w1.weak_count(), 1);
        assert_eq!(rc2.value, 7);

        let w2 = w1.clone();
        assert_eq!(RefCount::strong_count(&rc1), 2);
        assert_eq!(RefCount::strong_count(&rc2), 2);
        assert_eq!(w1.strong_count(), 2);
        assert_eq!(w2.strong_count(), 2);
        assert_eq!(RefCount::weak_count(&rc1), 2);
        assert_eq!(RefCount::weak_count(&rc2), 2);
        assert_eq!(w1.weak_count(), 2);
        assert_eq!(w2.weak_count(), 2);
        assert_eq!(rc2.value, 7);

        drop(rc1);
        assert_eq!(RefCount::strong_count(&rc2), 1);
        assert_eq!(w1.strong_count(), 1);
        assert_eq!(w2.strong_count(), 1);
        assert_eq!(RefCount::weak_count(&rc2), 2);
        assert_eq!(w1.weak_count(), 2);
        assert_eq!(w2.weak_count(), 2);
        assert!(!*dropped.borrow());
        assert!(w1.upgrade().is_some());
        assert!(w2.upgrade().is_some());

        drop(rc2);
        assert_eq!(w1.strong_count(), 0);
        assert_eq!(w2.strong_count(), 0);
        assert_eq!(w1.weak_count(), 2);
        assert_eq!(w2.weak_count(), 2);
        assert!(*dropped.borrow());
        assert!(w1.upgrade().is_none());
        assert!(w2.upgrade().is_none());

        drop(w1);
        assert_eq!(w2.strong_count(), 0);
        assert_eq!(w2.weak_count(), 1);
        assert!(*dropped.borrow());
        assert!(w2.upgrade().is_none());
    }

    #[test]
    fn test_try_unwrap() {
        let dropped = RefCell::new(false);

        let inner = Inner {
            value: 7,
            on_drop: || {
                *dropped.borrow_mut() = true;
            },
        };

        let rc1 = RefCount::new(inner);
        let rc2 = RefCount::clone(&rc1);
        let w = RefCount::downgrade(&rc1);

        let result = RefCount::try_unwrap(rc2);
        let Err(rc2) = result else {
            panic!("result should be Err(_)")
        };
        assert!(w.upgrade().is_some());

        drop(rc1);
        let result = RefCount::try_unwrap(rc2);
        let Ok(inner) = result else {
            panic!("result should be Ok(_)");
        };
        assert!(w.upgrade().is_none());
        assert_eq!(w.strong_count(), 0);
        assert_eq!(w.weak_count(), 1);
        assert_eq!(inner.value, 7);
        assert!(!*dropped.borrow());
    }

    #[test]
    fn test_ref_count_raw() {
        let dropped = RefCell::new(false);

        let inner = Inner {
            value: 7,
            on_drop: || {
                *dropped.borrow_mut() = true;
            },
        };

        let rc1 = RefCount::new(inner);
        let rc2 = RefCount::clone(&rc1);

        let ptr = RefCount::into_raw(rc1);
        assert_eq!(RefCount::strong_count(&rc2), 2);
        assert_eq!(RefCount::weak_count(&rc2), 0);
        assert_eq!(unsafe { ptr.as_ref() }.unwrap().value, 7);
        assert!(!*dropped.borrow());

        let rc1 = unsafe { RefCount::from_raw(ptr) };
        assert_eq!(RefCount::strong_count(&rc1), 2);
        assert_eq!(RefCount::strong_count(&rc2), 2);
        assert_eq!(RefCount::weak_count(&rc1), 0);
        assert_eq!(RefCount::weak_count(&rc2), 0);
        assert_eq!(rc1.value, 7);
        assert!(!*dropped.borrow());
    }

    #[test]
    fn test_weak_ref_raw() {
        let dropped = RefCell::new(false);

        let inner = Inner {
            value: 7,
            on_drop: || {
                *dropped.borrow_mut() = true;
            },
        };

        let rc1 = RefCount::new(inner);
        let w = RefCount::downgrade(&rc1);
        let ptr = w.into_raw();
        assert_eq!(RefCount::strong_count(&rc1), 1);
        assert_eq!(RefCount::weak_count(&rc1), 1);
        assert_eq!(unsafe { ptr.as_ref() }.unwrap().value, 7);
        assert!(!*dropped.borrow());

        let w = unsafe { WeakRef::from_raw(ptr) };
        assert_eq!(RefCount::strong_count(&rc1), 1);
        assert_eq!(w.strong_count(), 1);
        assert_eq!(RefCount::weak_count(&rc1), 1);
        assert_eq!(w.weak_count(), 1);
        assert_eq!(rc1.value, 7);
        assert!(!*dropped.borrow());

        let rc2 = w.upgrade().expect("Upgraded ref should be Some(_)");
        assert_eq!(RefCount::strong_count(&rc1), 2);
        assert_eq!(RefCount::strong_count(&rc2), 2);
        assert_eq!(w.strong_count(), 2);
        assert_eq!(RefCount::weak_count(&rc1), 1);
        assert_eq!(RefCount::weak_count(&rc2), 1);
        assert_eq!(w.weak_count(), 1);
        assert_eq!(rc2.value, 7);
        assert!(!*dropped.borrow());
    }

    #[test]
    fn test_as_ptr() {
        let dropped = RefCell::new(false);

        let inner = Inner {
            value: 7,
            on_drop: || {
                *dropped.borrow_mut() = true;
            },
        };

        let rc = RefCount::new(inner);
        let w = RefCount::downgrade(&rc);

        let rc_as_ptr = RefCount::as_ptr(&rc);
        let w_as_ptr = w.as_ptr();

        assert!(ptr::eq(rc_as_ptr, &raw const *rc));
        assert!(ptr::eq(w_as_ptr, &raw const *rc));
        assert_eq!(unsafe { &*rc_as_ptr }.value, 7);
    }

    #[test]
    fn test_get_mut() {
        let dropped = RefCell::new(false);

        let inner = Inner {
            value: 7,
            on_drop: || {
                *dropped.borrow_mut() = true;
            },
        };

        let mut rc1 = RefCount::new(inner);

        let result = RefCount::get_mut(&mut rc1);
        let Some(r) = result else {
            panic!("result should be Some(_)")
        };
        r.value += 1;

        let rc2 = RefCount::clone(&rc1);
        assert!(RefCount::get_mut(&mut rc1).is_none());
        drop(rc2);

        let w = RefCount::downgrade(&rc1);
        assert!(RefCount::get_mut(&mut rc1).is_none());
        drop(w);

        let result = RefCount::get_mut(&mut rc1);
        let Some(r) = result else {
            panic!("result should be Some(_)")
        };
        assert_eq!(r.value, 8);
    }
}
