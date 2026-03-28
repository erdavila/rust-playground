use std::cell::RefCell;
use std::rc::Rc;

struct ObjectUsage {
    accessed: bool,
    mutated: bool,
    moved: bool,
}
impl ObjectUsage {
    fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(ObjectUsage {
            accessed: false,
            mutated: false,
            moved: false,
        }))
    }
}

struct Usage(Rc<RefCell<ObjectUsage>>);
impl Usage {
    fn new() -> Self {
        Usage(ObjectUsage::new())
    }
    fn accessed(&self) -> bool {
        self.0.borrow().accessed
    }
    fn mutated(&self) -> bool {
        self.0.borrow().mutated
    }
    fn moved(&self) -> bool {
        self.0.borrow().moved
    }
    fn new_object(&self) -> Object {
        Object(Rc::clone(&self.0))
    }
}

struct Object(Rc<RefCell<ObjectUsage>>);
impl Object {
    fn access(&self) {
        self.0.borrow_mut().accessed = true;
    }
    fn mutate(&mut self) {
        self.0.borrow_mut().mutated = true;
    }
    fn r#move(self) {
        self.0.borrow_mut().moved = true;
    }
}

mod emulated_fn_once {
    use emulated_closures::{EmulatedFnOnce, emulated_fn_once};

    use crate::{Object, Usage};

    #[test]
    fn test_access() {
        let obj_ref_usage = Usage::new();
        let obj_ref = &obj_ref_usage.new_object();

        let closure = emulated_fn_once! {
            captures: { [obj_ref => &Object] },
            signature: () => (),
            |(obj_ref,), ()| {
                obj_ref.access();
            }
        };

        assert!(!obj_ref_usage.accessed());
        closure.call_once(());
        assert!(obj_ref_usage.accessed());
    }

    #[test]
    fn test_mutate() {
        let obj_mut_ref_usage = Usage::new();
        let obj_mut_ref = &mut obj_mut_ref_usage.new_object();

        let closure = emulated_fn_once! {
            captures: { [obj_mut_ref => &mut Object] },
            signature: () => (),
            |(obj_ref,), ()| {
                obj_ref.mutate();
            }
        };

        assert!(!obj_mut_ref_usage.mutated());
        closure.call_once(());
        assert!(obj_mut_ref_usage.mutated());
    }

    #[test]
    fn test_move() {
        let owned_usage = Usage::new();
        let owned = owned_usage.new_object();

        let moved_usage = Usage::new();
        let moved = moved_usage.new_object();

        let closure = emulated_fn_once! {
            captures: {
                [owned => Object],
                [moved => move Object],
            },
            signature: () => (),
            |(owned, moved), ()| {
                owned.r#move();
                moved.r#move();
            }
        };

        assert!(!owned_usage.moved());
        assert!(!moved_usage.moved());
        closure.call_once(());
        assert!(owned_usage.moved());
        assert!(moved_usage.moved());
    }

    #[test]
    fn test_arguments() {
        let mut x = 1;

        let closure = emulated_fn_once! {
            captures: {[ &mut x => &mut u32 ]},
            signature: (u32, u32) => u32,
            |(x,), (a, b)| {
                *x += a;
                b
            }
        };

        let result = closure.call_once((3, 5));

        assert_eq!(x, 4);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_no_captures() {
        let closure = emulated_fn_once! {
            captures: {},
            signature: (u32,) => u32,
            |(), (n,)| {
                2 * n
            }
        };

        let result = closure.call_once((7,));

        assert_eq!(result, 14);
    }
}

mod emulated_fn_mut {
    use emulated_closures::{EmulatedFnMut, emulated_fn_mut};

    use crate::{Object, Usage};

    #[test]
    fn test_access() {
        let obj_ref_usage = Usage::new();
        let obj_ref = &obj_ref_usage.new_object();

        let mut closure = emulated_fn_mut! {
            captures: { [obj_ref => &Object] },
            signature: () => (),
            |(obj_ref,), ()| {
                obj_ref.access();
            }
        };

        assert!(!obj_ref_usage.accessed());
        closure.call_mut(());
        assert!(obj_ref_usage.accessed());
    }

    #[test]
    fn test_mutate() {
        let obj_mut_ref_usage = Usage::new();
        let obj_mut_ref = &mut obj_mut_ref_usage.new_object();

        let mut closure = emulated_fn_mut! {
            captures: { [obj_mut_ref => &mut Object] },
            signature: () => (),
            |(obj_ref,), ()| {
                obj_ref.mutate();
            }
        };

        assert!(!obj_mut_ref_usage.mutated());
        closure.call_mut(());
        assert!(obj_mut_ref_usage.mutated());
    }

    #[test]
    fn test_moved() {
        let moved_usage = Usage::new();
        let moved = moved_usage.new_object();

        let mut closure = emulated_fn_mut! {
            captures: { [moved => move Object] },
            signature: () => (),
            |(moved,), ()| {
                moved.access();
                moved.mutate();
            }
        };

        assert!(!moved_usage.accessed());
        assert!(!moved_usage.mutated());
        closure.call_mut(());
        assert!(moved_usage.accessed());
        assert!(moved_usage.mutated());
    }

    #[test]
    fn test_arguments() {
        let mut x = 1;

        let mut closure = emulated_fn_mut! {
            captures: {[ &mut x => &mut u32 ]},
            signature: (u32, u32) => u32,
            |(x,), (a, b)| {
                **x += a;
                b
            }
        };

        let result = closure.call_mut((3, 5));

        assert_eq!(x, 4);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_no_captures() {
        let mut closure = emulated_fn_mut! {
            captures: {},
            signature: (u32,) => u32,
            |(), (n,)| {
                2 * n
            }
        };

        let result = closure.call_mut((7,));

        assert_eq!(result, 14);
    }
}

mod emulated_fn {
    use emulated_closures::{EmulatedFn, emulated_fn};

    use crate::{Object, Usage};

    #[test]
    fn test_access() {
        let obj_ref_usage = Usage::new();
        let obj_ref = &obj_ref_usage.new_object();

        let closure = emulated_fn! {
            captures: { [obj_ref => &Object] },
            signature: () => (),
            |(obj_ref,), ()| {
                obj_ref.access();
            }
        };

        assert!(!obj_ref_usage.accessed());
        closure.call(());
        assert!(obj_ref_usage.accessed());
    }

    #[test]
    fn test_moved() {
        let moved_usage = Usage::new();
        let moved = moved_usage.new_object();

        let closure = emulated_fn! {
            captures: { [moved => move Object] },
            signature: () => (),
            |(moved,), ()| {
                moved.access();
            }
        };

        assert!(!moved_usage.accessed());
        closure.call(());
        assert!(moved_usage.accessed());
    }

    #[test]
    fn test_arguments() {
        let x = 1;

        let closure = emulated_fn! {
            captures: {[ &x => &u32 ]},
            signature: (u32, u32) => u32,
            |(x,), (a, b)| {
                *x + a + b
            }
        };

        let result = closure.call((3, 5));

        assert_eq!(result, 9);
    }

    #[test]
    fn test_no_captures() {
        let closure = emulated_fn! {
            captures: {},
            signature: (u32,) => u32,
            |(), (n,)| {
                2 * n
            }
        };

        let result = closure.call((7,));

        assert_eq!(result, 14);
    }
}
