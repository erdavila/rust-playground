pub trait EmulatedFnOnce<Args> {
    type Output;

    fn call_once(self, args: Args) -> Self::Output;
}

pub trait EmulatedFnMut<Args>: EmulatedFnOnce<Args> {
    fn call_mut(&mut self, args: Args) -> Self::Output;
}

#[macro_export]
macro_rules! emulated_fn_once {
    (
        captures: {
            $( [ $($capture:tt)* ] ),*
            $(,)?
        },
        signature: ( $($arg:ty),* $(,)? ) => $out:ty,
        $body:expr
    ) => {
        {
            #[allow(non_camel_case_types)]
            struct impl_EmulatedFnOnce<'lt> {
                captures: ( $( $crate::__emulated_closures__fn_once_capture_type!( $($capture)* ) , )* ),
                _phantom: std::marker::PhantomData<&'lt ()>,
            }

            impl<'lt> $crate::EmulatedFnOnce<( $( $arg , )* )> for impl_EmulatedFnOnce<'lt> {
                type Output = $out;

                fn call_once(self, args: ( $( $arg , )* )) -> Self::Output {
                    let body: fn(( $( $crate::__emulated_closures__fn_once_capture_type!( $($capture)* ) , )* ), ( $( $arg , )* )) -> $out = $body;
                    body(self.captures, args)
                }
            }

            impl_EmulatedFnOnce {
                captures: ( $( $crate::__emulated_closures__capture_expr!( $($capture)* ) , )* ),
                _phantom: std::marker::PhantomData,
            }
        }
    };
}

#[macro_export]
macro_rules! emulated_fn_mut {
    (
        captures: {
            $( [ $($capture:tt)* ] ),*
            $(,)?
        },
        signature: ( $($arg:ty),* $(,)? ) => $out:ty,
        $body:expr
    ) => {
        {
            #[allow(non_camel_case_types)]
            struct impl_EmulatedFnMut<'lt> {
                captures: ( $( $crate::__emulated_closures__fn_mut_capture_type!( $($capture)* ) , )* ),
                _phantom: std::marker::PhantomData<&'lt ()>,
            }

            impl<'lt> $crate::EmulatedFnMut<( $( $arg , )* )> for impl_EmulatedFnMut<'lt> {
                fn call_mut(&mut self, args: ( $( $arg , )* )) -> Self::Output {
                    let body: fn(&mut ( $( $crate::__emulated_closures__fn_mut_capture_type!( $($capture)* ) , )* ), ( $( $arg , )* )) -> $out = $body;
                    body(&mut self.captures, args)
                }
            }

            impl<'lt> $crate::EmulatedFnOnce<( $( $arg , )* )> for impl_EmulatedFnMut<'lt> {
                type Output = $out;

                fn call_once(mut self, args: ( $( $arg , )* )) -> Self::Output {
                    self.call_mut(args)
                }
            }

            impl_EmulatedFnMut {
                captures: ( $( $crate::__emulated_closures__capture_expr!( $($capture)* ) , )* ),
                _phantom: std::marker::PhantomData,
            }
        }
    };
}

#[macro_export]
macro_rules! __emulated_closures__fn_once_capture_type {
    ($expr:expr => &mut $ty:ty) => { &'lt mut $ty };
    ($expr:expr => & $ty:ty) => { &'lt $ty };
    ($expr:expr => move $ty:ty) => { $ty };
    ($expr:expr => $ty:ty) => { $ty };
}

#[macro_export]
macro_rules! __emulated_closures__fn_mut_capture_type {
    ($expr:expr => &mut $ty:ty) => { &'lt mut $ty };
    ($expr:expr => & $ty:ty) => { &'lt $ty };
    ($expr:expr => move $ty:ty) => { $ty };
}

#[macro_export]
macro_rules! __emulated_closures__capture_expr {
    ($expr:expr => $($tt:tt)*) => {
        $expr
    };
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

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
        use crate::{tests::Object, EmulatedFnOnce};

        use super::Usage;

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
        use crate::{tests::Object, EmulatedFnMut};

        use super::Usage;

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
}
