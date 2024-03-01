use std::{cell::RefCell, rc::Rc};

use crate::refs::{Ref, RefMut};

pub struct SharedMutableOwnership<T>(Rc<RefCell<T>>);

impl<T> Clone for SharedMutableOwnership<T> {
    fn clone(&self) -> Self {
        Self(Clone::clone(&self.0))
    }
}

impl<T> SharedMutableOwnership<T> {
    pub fn new(value: T) -> SharedMutableOwnership<T> {
        SharedMutableOwnership(Rc::new(RefCell::new(value)))
    }

    pub fn get_ref(&self) -> Ref<T> {
        Ref(self.refcell().borrow())
    }

    pub fn get_mut(&mut self) -> RefMut<T> {
        RefMut(self.refcell().borrow_mut())
    }

    fn refcell(&self) -> &RefCell<T> {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::SharedMutableOwnership;
    use crate::tests::{Usage, Value};

    #[test]
    fn immutable_x_immutable() {
        let (val, val_usage) = Value::new_with_usage();

        let shared1 = SharedMutableOwnership::new(val);
        let shared2 = shared1.clone();

        shared1.get_ref().access();

        shared2.get_ref().access();

        assert_eq!(
            *val_usage.borrow(),
            Usage {
                accesses: 2,
                mutations: 0,
                moves: 0,
            }
        );
    }

    #[test]
    fn immutable_x_mutable() {
        let (val, val_usage) = Value::new_with_usage();

        let shared1 = SharedMutableOwnership::new(val);
        let mut shared2 = shared1.clone();

        shared1.get_ref().access();

        shared2.get_mut().mutate();

        assert_eq!(
            *val_usage.borrow(),
            Usage {
                accesses: 1,
                mutations: 1,
                moves: 0,
            }
        );
    }

    #[test]
    fn mutable_x_immutable() {
        let (val, val_usage) = Value::new_with_usage();

        let mut shared1 = SharedMutableOwnership::new(val);
        let shared2 = shared1.clone();

        shared1.get_mut().mutate();

        shared2.get_ref().access();

        assert_eq!(
            *val_usage.borrow(),
            Usage {
                accesses: 1,
                mutations: 1,
                moves: 0,
            }
        );
    }

    #[test]
    fn mutable_x_mutable() {
        let (val, val_usage) = Value::new_with_usage();

        let mut shared1 = SharedMutableOwnership::new(val);
        let mut shared2 = shared1.clone();

        shared1.get_mut().mutate();

        shared2.get_mut().mutate();

        assert_eq!(
            *val_usage.borrow(),
            Usage {
                accesses: 0,
                mutations: 2,
                moves: 0,
            }
        );
    }
}
