use std::cell::RefCell;
use std::rc::Rc;

pub(crate) struct Node {
    connected_to: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    pub(crate) fn new() -> Self {
        count::inc();
        Node {
            connected_to: Vec::new(),
        }
    }

    pub(crate) fn connected_to(&mut self) -> &mut Vec<Rc<RefCell<Node>>> {
        &mut self.connected_to
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        count::dec();
    }
}

pub(crate) mod count {
    static mut VALUE: usize = 0;

    fn value() -> &'static mut usize {
        #[allow(static_mut_refs)]
        unsafe {
            &mut VALUE
        }
    }

    pub(crate) fn reset() {
        *value() = 0;
    }

    pub(crate) fn inc() {
        *value() += 1;
    }

    pub(crate) fn dec() {
        *value() -= 1;
    }

    #[must_use]
    pub(crate) fn get() -> usize {
        *value()
    }
}
