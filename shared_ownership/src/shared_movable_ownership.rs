use std::{
    cell::{self, RefCell},
    fmt::Debug,
    rc::Rc,
};

use crate::{AlreadyMutablyBorrowed, Ref, RefMut};

pub struct SharedMovableOwnership<T>(Rc<RefCell<Option<T>>>);

impl<T> Clone for SharedMovableOwnership<T> {
    fn clone(&self) -> Self {
        Self(Clone::clone(&self.0))
    }
}

impl<T> Debug for SharedMovableOwnership<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("SharedMovableOwnership")
            .field(&self.0)
            .finish()
    }
}

impl<T> SharedMovableOwnership<T> {
    pub fn new(value: T) -> SharedMovableOwnership<T> {
        SharedMovableOwnership(Rc::new(RefCell::new(Some(value))))
    }

    #[must_use]
    pub fn get_ref(&self) -> Option<Ref<'_, T>> {
        let refcell_ref = self.refcell().borrow();
        refcell_ref.is_some().then(|| {
            Ref(cell::Ref::map(refcell_ref, |option| {
                option.as_ref().unwrap()
            }))
        })
    }

    pub fn get_mut(&mut self) -> Option<RefMut<'_, T>> {
        let refcell_refmut = self.refcell().borrow_mut();
        refcell_refmut.is_some().then(|| {
            RefMut(cell::RefMut::map(refcell_refmut, |option| {
                option.as_mut().unwrap()
            }))
        })
    }

    #[must_use]
    pub fn r#move(self) -> Option<T> {
        self.refcell().borrow_mut().take()
    }

    pub fn try_get_ref(&self) -> Result<Option<Ref<'_, T>>, AlreadyMutablyBorrowed> {
        self.refcell()
            .try_borrow()
            .map(|refcell_ref| {
                refcell_ref.is_some().then(|| {
                    Ref(cell::Ref::map(refcell_ref, |option| {
                        option.as_ref().unwrap()
                    }))
                })
            })
            .map_err(|_| AlreadyMutablyBorrowed)
    }

    pub fn try_get_mut(&mut self) -> Result<Option<RefMut<'_, T>>, AlreadyMutablyBorrowed> {
        self.refcell()
            .try_borrow_mut()
            .map(|refcell_refmut| {
                refcell_refmut.is_some().then(|| {
                    RefMut(cell::RefMut::map(refcell_refmut, |option| {
                        option.as_mut().unwrap()
                    }))
                })
            })
            .map_err(|_| AlreadyMutablyBorrowed)
    }

    pub fn try_move(self) -> Result<Option<T>, Self> {
        self.refcell()
            .try_borrow_mut()
            .map(|mut refcell_refmut| refcell_refmut.take())
            .map_err(|_| self)
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

    #[test]
    fn try_get_when_not_borrowed() {
        let (val, val_usage) = Value::new_with_usage();

        let shared = SharedMovableOwnership::new(val);

        if let Some(val) = shared.try_get_ref().unwrap() {
            val.access();
        }

        assert_eq!(
            *val_usage.borrow(),
            Usage {
                accesses: 1,
                mutations: 0,
                moves: 0,
            }
        );
    }

    #[test]
    fn try_get_when_immutably_borrowed() {
        let (val, val_usage) = Value::new_with_usage();

        let shared1 = SharedMovableOwnership::new(val);
        let shared2 = shared1.clone();

        let _val_ref = shared1.get_ref();

        if let Some(val) = shared2.try_get_ref().unwrap() {
            val.access();
        }

        assert_eq!(
            *val_usage.borrow(),
            Usage {
                accesses: 1,
                mutations: 0,
                moves: 0,
            }
        );
    }

    #[test]
    fn try_get_when_mutably_borrowed() {
        let (val, _val_usage) = Value::new_with_usage();

        let mut shared1 = SharedMovableOwnership::new(val);
        let shared2 = shared1.clone();

        let _val_ref = shared1.get_mut();

        assert!(shared2.try_get_ref().is_err());
    }

    #[test]
    fn try_get_mut_when_not_borrowed() {
        let (val, val_usage) = Value::new_with_usage();

        let mut shared = SharedMovableOwnership::new(val);

        if let Some(mut val) = shared.try_get_mut().unwrap() {
            val.mutate();
        }

        assert_eq!(
            *val_usage.borrow(),
            Usage {
                accesses: 0,
                mutations: 1,
                moves: 0,
            }
        );
    }

    #[test]
    fn try_get_mut_when_immutably_borrowed() {
        let (val, _val_usage) = Value::new_with_usage();

        let shared1 = SharedMovableOwnership::new(val);
        let mut shared2 = shared1.clone();

        let _val_ref = shared1.get_ref();

        assert!(shared2.try_get_mut().is_err());
    }

    #[test]
    fn try_get_mut_when_mutably_borrowed() {
        let (val, _val_usage) = Value::new_with_usage();

        let mut shared1 = SharedMovableOwnership::new(val);
        let mut shared2 = shared1.clone();

        let _val_ref = shared1.get_mut();

        assert!(shared2.try_get_mut().is_err());
    }

    #[test]
    fn try_move_when_not_borrowed() {
        let (val, val_usage) = Value::new_with_usage();

        let shared = SharedMovableOwnership::new(val);

        if let Some(val) = shared.try_move().unwrap() {
            val.r#move();
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
    fn try_move_when_immutably_borrowed() {
        let (val, _val_usage) = Value::new_with_usage();

        let shared1 = SharedMovableOwnership::new(val);
        let shared2 = shared1.clone();

        let _val_ref = shared1.get_ref();

        assert!(shared2.try_move().is_err());
    }

    #[test]
    fn try_move_when_mutably_borrowed() {
        let (val, _val_usage) = Value::new_with_usage();

        let mut shared1 = SharedMovableOwnership::new(val);
        let shared2 = shared1.clone();

        let _val_ref = shared1.get_mut();

        assert!(shared2.try_move().is_err());
    }
}
