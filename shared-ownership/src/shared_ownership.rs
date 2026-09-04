use std::rc::Rc;

pub struct SharedOwnership<T>(Rc<T>);

impl<T> Clone for SharedOwnership<T> {
    fn clone(&self) -> Self {
        Self(Clone::clone(&self.0))
    }
}

impl<T> SharedOwnership<T> {
    pub fn new(value: T) -> SharedOwnership<T> {
        SharedOwnership(Rc::new(value))
    }

    #[must_use]
    pub fn get_ref(&self) -> &T {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::SharedOwnership;
    use crate::tests::{Usage, Value};

    #[test]
    fn immutable_x_immutable() {
        let (val, val_usage) = Value::new_with_usage();

        let shared1 = SharedOwnership::new(val);
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
}
