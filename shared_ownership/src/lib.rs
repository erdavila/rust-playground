mod shared_ownership;

pub use shared_ownership::*;

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    #[derive(Default, PartialEq, Eq, Debug)]
    pub(crate) struct Usage {
        pub(crate) accesses: u32,
        pub(crate) mutations: u32,
        pub(crate) moves: u32,
    }

    pub(crate) struct Value(Rc<RefCell<Usage>>);

    impl Value {
        pub(crate) fn new_with_usage() -> (Self, Rc<RefCell<Usage>>) {
            let usage = Rc::new(RefCell::new(Usage::default()));
            let val = Value(Rc::clone(&usage));
            (val, usage)
        }

        pub(crate) fn access(&self) {
            self.0.borrow_mut().accesses += 1;
        }
    }
}
