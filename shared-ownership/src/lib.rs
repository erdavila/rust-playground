#![expect(clippy::missing_errors_doc, clippy::missing_panics_doc)]

mod refs;
mod shared_movable_ownership;
mod shared_mutable_ownership;
mod shared_ownership;

use std::error::Error;
use std::fmt::Display;

pub use refs::*;
pub use shared_movable_ownership::*;
pub use shared_mutable_ownership::*;
pub use shared_ownership::*;

#[derive(Debug)]
pub struct AlreadyMutablyBorrowed;
impl Error for AlreadyMutablyBorrowed {}
impl Display for AlreadyMutablyBorrowed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Default, PartialEq, Eq, Debug)]
    pub(crate) struct Usage {
        pub(crate) accesses: u32,
        pub(crate) mutations: u32,
        pub(crate) moves: u32,
    }

    #[derive(Debug)]
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

        pub(crate) fn mutate(&mut self) {
            self.0.borrow_mut().mutations += 1;
        }

        pub(crate) fn r#move(self) {
            self.0.borrow_mut().moves += 1;
        }
    }
}
