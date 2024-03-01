pub use std::cell::{Ref, RefMut};
use std::{cell::RefCell, rc::Rc};

pub struct SharedMovableOwnership<T>(Rc<RefCell<Option<T>>>);

impl<T> Clone for SharedMovableOwnership<T> {
    fn clone(&self) -> Self {
        Self(Clone::clone(&self.0))
    }
}

impl<T> SharedMovableOwnership<T> {
    pub fn new(value: T) -> SharedMovableOwnership<T> {
        SharedMovableOwnership(Rc::new(RefCell::new(Some(value))))
    }

    pub fn get_ref(&self) -> Option<Ref<T>> {
        let refcell_ref = self.refcell().borrow();
        refcell_ref
            .is_some()
            .then(|| Ref::map(refcell_ref, |option| option.as_ref().unwrap()))
    }

    pub fn get_mut(&mut self) -> Option<RefMut<T>> {
        let refcell_refmut = self.refcell().borrow_mut();
        refcell_refmut
            .is_some()
            .then(|| RefMut::map(refcell_refmut, |option| option.as_mut().unwrap()))
    }

    pub fn r#move(self) -> Option<T> {
        self.refcell().borrow_mut().take()
    }

    fn refcell(&self) -> &RefCell<Option<T>> {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::SharedMovableOwnership;
    use crate::tests::{Usage, Value};

    #[test]
    fn immutable_x_immutable() {
        let (val, val_usage) = Value::new_with_usage();

        let shared1 = SharedMovableOwnership::new(val);
        let shared2 = shared1.clone();

        if let Some(val) = shared1.get_ref() {
            val.access();
        }

        if let Some(val) = shared2.get_ref() {
            val.access();
        }

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

        let shared1 = SharedMovableOwnership::new(val);
        let mut shared2 = shared1.clone();

        if let Some(val) = shared1.get_ref() {
            val.access();
        }

        if let Some(mut val) = shared2.get_mut() {
            val.mutate();
        }

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
    fn immutable_x_move() {
        let (val, val_usage) = Value::new_with_usage();

        let shared1 = SharedMovableOwnership::new(val);
        let shared2 = shared1.clone();

        if let Some(val) = shared1.get_ref() {
            val.access();
        }

        if let Some(val) = shared2.r#move() {
            val.r#move();
        }

        assert_eq!(
            *val_usage.borrow(),
            Usage {
                accesses: 1,
                mutations: 0,
                moves: 1,
            }
        );
    }

    #[test]
    fn mutable_x_immutable() {
        let (val, val_usage) = Value::new_with_usage();

        let mut shared1 = SharedMovableOwnership::new(val);
        let shared2 = shared1.clone();

        if let Some(mut val) = shared1.get_mut() {
            val.mutate();
        }

        if let Some(val) = shared2.get_ref() {
            val.access();
        }

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

        let mut shared1 = SharedMovableOwnership::new(val);
        let mut shared2 = shared1.clone();

        if let Some(mut val) = shared1.get_mut() {
            val.mutate();
        }

        if let Some(mut val) = shared2.get_mut() {
            val.mutate();
        }

        assert_eq!(
            *val_usage.borrow(),
            Usage {
                accesses: 0,
                mutations: 2,
                moves: 0,
            }
        );
    }

    #[test]
    fn mutable_x_move() {
        let (val, val_usage) = Value::new_with_usage();

        let mut shared1 = SharedMovableOwnership::new(val);
        let shared2 = shared1.clone();

        if let Some(mut val) = shared1.get_mut() {
            val.mutate();
        }

        if let Some(val) = shared2.r#move() {
            val.r#move();
        }

        assert_eq!(
            *val_usage.borrow(),
            Usage {
                accesses: 0,
                mutations: 1,
                moves: 1,
            }
        );
    }

    #[test]
    fn move_x_immutable() {
        let (val, val_usage) = Value::new_with_usage();

        let shared1 = SharedMovableOwnership::new(val);
        let shared2 = shared1.clone();

        if let Some(val) = shared1.r#move() {
            val.r#move();
        }

        if let Some(_val) = shared2.get_ref() {
            assert!(false);
        }

        assert_eq!(
            *val_usage.borrow(),
            Usage {
                accesses: 0,
                mutations: 0,
                moves: 1,
            }
        );
    }

    #[test]
    fn move_x_mutable() {
        let (val, val_usage) = Value::new_with_usage();

        let shared1 = SharedMovableOwnership::new(val);
        let mut shared2 = shared1.clone();

        if let Some(val) = shared1.r#move() {
            val.r#move();
        }

        if let Some(_val) = shared2.get_mut() {
            assert!(false);
        }

        assert_eq!(
            *val_usage.borrow(),
            Usage {
                accesses: 0,
                mutations: 0,
                moves: 1,
            }
        );
    }

    #[test]
    fn move_x_move() {
        let (val, val_usage) = Value::new_with_usage();

        let shared1 = SharedMovableOwnership::new(val);
        let shared2 = shared1.clone();

        if let Some(val) = shared1.r#move() {
            val.r#move();
        }

        if let Some(_val) = shared2.r#move() {
            assert!(false);
        }

        assert_eq!(
            *val_usage.borrow(),
            Usage {
                accesses: 0,
                mutations: 0,
                moves: 1,
            }
        );
    }
}
