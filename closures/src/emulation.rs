use crate::Object;

trait EmulatedFnOnce<Args> {
    type Output;

    fn call_once(self, args: Args) -> Self::Output;
}

trait EmulatedFnMut<Args>: EmulatedFnOnce<Args> {
    fn call_mut(&mut self, args: Args) -> Self::Output;
}

trait EmulatedFn<Args>: EmulatedFnMut<Args> {
    fn call(&self, args: Args) -> Self::Output;
}

pub fn test() {
    test_fn();
    test_fn_mut();
    test_fn_once();

    test_moving_fn();
    test_moving_fn_mut();
    test_moving_fn_once();
}

fn test_fn() {
    let obj_ref = &Object;
    let obj_mut_ref = &mut Object;
    let mut owned = Object;

    let closure = {
        struct EmulatedFn<'a>(&'a Object, &'a Object, &'a Object);
        impl self::EmulatedFnOnce<()> for EmulatedFn<'_> {
            type Output = ();
            fn call_once(mut self, args: ()) -> Self::Output {
                self.call_mut(args);
            }
        }
        impl self::EmulatedFnMut<()> for EmulatedFn<'_> {
            fn call_mut(&mut self, args: ()) -> Self::Output {
                self.call(args);
            }
        }
        impl self::EmulatedFn<()> for EmulatedFn<'_> {
            fn call(&self, (): ()) -> Self::Output {
                let EmulatedFn(obj_ref, obj_mut_ref, owned) = self;

                obj_ref.access();
                obj_mut_ref.access();
                owned.access();

                // Mutability required.
                // obj_mut_ref.mutate();
                // Mutability required.
                // owned.mutate();

                // Ownership required.
                // owned.r#move();
            }
        }
        EmulatedFn(
            // Immutable reference captured as immutable reference.
            obj_ref,
            // Mutable reference captured as immutable reference.
            obj_mut_ref,
            // Owned value captured as immutable reference.
            &owned,
        )
    };

    let closure = accept_fn(closure);
    let closure = accept_fn_mut(closure);
    let closure = accept_fn_once(closure);

    obj_ref.access();
    obj_mut_ref.access();
    owned.access();

    // Cannot borrow as mutable because it is borrowed as mutable by the closure.
    // obj_mut_ref.mutate();
    // Cannot borrow as mutable because it is borrowed as immutable by the closure.
    // owned.mutate();

    // Cannot move because it is borrowed by the closure.
    // owned.r#move();

    closure.call(());
    // Can be called again.
    closure.call(());

    obj_ref.access();
    obj_mut_ref.access();
    owned.access();

    obj_mut_ref.mutate();
    owned.mutate();

    owned.r#move();
}

fn test_fn_mut() {
    let obj_ref = &Object;
    let obj_mut_ref = &mut Object;
    let mut owned = Object;

    let closure = {
        struct EmulatedFnMut<'a>(&'a Object, &'a mut Object, &'a mut Object);
        impl self::EmulatedFnOnce<()> for EmulatedFnMut<'_> {
            type Output = ();
            fn call_once(mut self, args: ()) -> Self::Output {
                self.call_mut(args);
            }
        }
        impl self::EmulatedFnMut<()> for EmulatedFnMut<'_> {
            fn call_mut(&mut self, (): ()) -> Self::Output {
                let EmulatedFnMut(obj_ref, obj_mut_ref, owned) = self;

                obj_ref.access();
                obj_mut_ref.access();
                owned.access();

                obj_mut_ref.mutate();
                owned.mutate();

                // Cannot move because it is borrowed.
                // owned.r#move();
            }
        }
        EmulatedFnMut(
            // Immutable reference captured as immutable reference.
            obj_ref,
            // Mutable reference captured as mutable reference.
            obj_mut_ref,
            // Owned value captured as mutable reference.
            &mut owned,
        )
    };

    // Closure that mutates captured value cannot be FnClosure.
    // let closure = accept_fn(closure);
    let closure = accept_fn_mut(closure);
    let closure = accept_fn_once(closure);

    obj_ref.access();
    // Cannot borrow as immutable because it is borrowed as mutable by the closure.
    // obj_mut_ref.access();
    // Cannot borrow as immutable because it is borrowed as mutable by the closure.
    // owned.access();

    // Cannot borrow as mutable because it is borrowed as mutable by the closure.
    // obj_mut_ref.mutate();
    // Cannot borrow as mutable because it is borrowed as immutable by the closure.
    // owned.mutate();

    // Cannot move because it is borrowed by the closure.
    // owned.r#move();

    let mut closure = closure;
    closure.call_mut(());
    // Can be called again.
    closure.call_mut(());

    obj_ref.access();
    obj_mut_ref.access();
    owned.access();

    obj_mut_ref.mutate();
    owned.mutate();

    owned.r#move();
}

fn test_fn_once() {
    let obj_ref = &Object;
    let obj_mut_ref = &mut Object;
    // EMULATION DISCREPANCY: Mutability is not required.
    let owned = Object;

    let closure = {
        struct EmulatedFnOnce<'a>(&'a Object, &'a mut Object, Object);
        impl self::EmulatedFnOnce<()> for EmulatedFnOnce<'_> {
            type Output = ();
            fn call_once(self, (): ()) -> Self::Output {
                let EmulatedFnOnce(
                    obj_ref,
                    obj_mut_ref,
                    // EMULATION: Mutability must be signaled here.
                    mut owned,
                ) = self;

                obj_ref.access();
                obj_mut_ref.access();
                owned.access();

                obj_mut_ref.mutate();
                owned.mutate();

                owned.r#move();
            }
        }
        EmulatedFnOnce(
            // Immutable reference captured as immutable reference.
            obj_ref,
            // Mutable reference captured as mutable reference.
            obj_mut_ref,
            // Owned value captured by moving.
            owned,
        )
    };

    // Closure that uses ownership of captured value cannot be FnClosure.
    // let closure = accept_fn(closure);
    // Closure that uses ownership of captured value cannot be FnMutClosure.
    // let closure = accept_fn_mut(closure);
    let closure = accept_fn_once(closure);

    obj_ref.access();
    // Cannot borrow as immutable because it is borrowed as mutable by the closure.
    // obj_mut_ref.access();
    // Cannot borrow because it is moved by the closure.
    // owned.access();

    // Cannot borrow as mutable because it is borrowed as mutable by the closure.
    // obj_mut_ref.mutate();
    // Cannot borrow because it is moved by the closure.
    // owned.mutate();

    // Cannot move because it is moved by the closure.
    // owned.r#move();

    closure.call_once(());
    // Cannot be called again.
    // closure.call_once(());

    obj_ref.access();
    obj_mut_ref.access();
    // Cannot borrow because it was moved by the closure.
    // owned.access();

    obj_mut_ref.mutate();
    // Cannot borrow because it was moved by the closure.
    // owned.mutate();

    // Cannot borrow because it was moved by the closure.
    // owned.r#move();
}

fn test_moving_fn() {
    let obj_ref = &Object;
    let obj_mut_ref = &mut Object;
    let owned = Object;

    let closure = {
        struct EmulatedMovingFn<'a>(&'a Object, &'a Object, Object);
        impl self::EmulatedFnOnce<()> for EmulatedMovingFn<'_> {
            type Output = ();
            fn call_once(mut self, args: ()) -> Self::Output {
                self.call_mut(args);
            }
        }
        impl self::EmulatedFnMut<()> for EmulatedMovingFn<'_> {
            fn call_mut(&mut self, args: ()) -> Self::Output {
                self.call(args);
            }
        }
        impl self::EmulatedFn<()> for EmulatedMovingFn<'_> {
            fn call(&self, (): ()) -> Self::Output {
                let EmulatedMovingFn(obj_ref, obj_mut_ref, owned) = self;

                obj_ref.access();
                obj_mut_ref.access();
                owned.access();

                // Cannot borrow as mutable because it is borrowed as immutable.
                // obj_mut_ref.mutate();
                // Cannot borrow as mutable because it is borrowed as immutable.
                // owned.mutate();

                // Cannot move because it is borrowed.
                // owned.r#move();
            }
        }
        EmulatedMovingFn(
            // Immutable reference captured as immutable reference.
            obj_ref,
            // Immutable reference captured as immutable reference.
            obj_mut_ref,
            // Owned value captured by moving.
            owned,
        )
    };

    let closure = accept_fn(closure);
    let closure = accept_fn_mut(closure);
    let closure = accept_fn_once(closure);

    obj_ref.access();
    // EMULATION DISCREPANCY: mutable reference should have been moved into.
    // obj_mut_ref.access();
    // Moved into closure.
    // closure owned.access();

    closure.call(());
    // Can be called again.
    closure.call(());

    obj_ref.access();
    // EMULATION DISCREPANCY: mutable reference should have been moved into.
    // obj_mut_ref.access();
    // Moved into closure.
    // closure owned.access();
}

fn test_moving_fn_mut() {
    let obj_ref = &Object;
    let obj_mut_ref = &mut Object;
    // EMULATION DISCREPANCY: Mutability is not required.
    let owned = Object;

    let closure = {
        struct EmulatedMovingFnMut<'a>(&'a Object, &'a mut Object, Object);
        impl self::EmulatedFnOnce<()> for EmulatedMovingFnMut<'_> {
            type Output = ();
            fn call_once(mut self, args: ()) -> Self::Output {
                self.call_mut(args);
            }
        }
        impl self::EmulatedFnMut<()> for EmulatedMovingFnMut<'_> {
            fn call_mut(&mut self, (): ()) -> Self::Output {
                let EmulatedMovingFnMut(obj_ref, obj_mut_ref, owned) = self;

                obj_ref.access();
                obj_mut_ref.access();
                owned.access();

                obj_mut_ref.mutate();
                owned.mutate();

                // Cannot move because it is borrowed.
                // owned.r#move();
            }
        }
        EmulatedMovingFnMut(
            // Immutable reference captured as immutable reference.
            obj_ref,
            // Mutable reference captured as mutable reference.
            obj_mut_ref,
            // Owned value captured by moving.
            owned,
        )
    };

    // Closure that mutates captured value cannot be FnClosure.
    // let closure = accept_fn(closure);
    let closure = accept_fn_mut(closure);
    let closure = accept_fn_once(closure);

    obj_ref.access();
    // Moved into closure.
    // obj_mut_ref.access();
    // Moved into closure.
    // owned.access();

    let mut closure = closure;
    closure.call_mut(());
    // Can be called again.
    closure.call_mut(());

    obj_ref.access();
    // EMULATION DISCREPANCY: mutable reference should have been moved into.
    // obj_mut_ref.access();
    // Moved into closure.
    // closure owned.access();
}

fn test_moving_fn_once() {
    let obj_ref = &Object;
    let obj_mut_ref = &mut Object;
    // EMULATION DISCREPANCY: Mutability is not required.
    let owned = Object;

    let closure = {
        struct EmulatedMovingFnOnce<'a>(&'a Object, &'a mut Object, Object);
        impl self::EmulatedFnOnce<()> for EmulatedMovingFnOnce<'_> {
            type Output = ();
            fn call_once(self, (): ()) -> Self::Output {
                let EmulatedMovingFnOnce(
                    obj_ref,
                    obj_mut_ref,
                    // EMULATION: Mutability must be signaled here.
                    mut owned,
                ) = self;

                obj_ref.access();
                obj_mut_ref.access();
                owned.access();

                obj_mut_ref.mutate();
                owned.mutate();

                owned.r#move();
            }
        }
        EmulatedMovingFnOnce(
            // Immutable reference captured as immutable reference.
            obj_ref,
            // Mutable reference captured as mutable reference.
            obj_mut_ref,
            // Owned value captured by moving.
            owned,
        )
    };

    // Closure that uses ownership of captured value cannot be FnClosure.
    // let closure = accept_fn(closure);
    // Closure that uses ownership of captured value cannot be FnMutClosure.
    // let closure = accept_fn_mut(closure);
    let closure = accept_fn_once(closure);

    obj_ref.access();
    // Moved into closure.
    // obj_mut_ref.access();
    // Moved into closure.
    // owned.access();

    closure.call_once(());
    // Cannot be called again.
    // closure.call_once(());

    obj_ref.access();
    // EMULATION DISCREPANCY: mutable reference should have been moved into.
    // obj_mut_ref.access();
    // Moved into closure.
    // closure owned.access();
}

fn accept_fn<Args, F: EmulatedFn<Args>>(f: F) -> F {
    f
}

fn accept_fn_mut<Args, F: EmulatedFnMut<Args>>(f: F) -> F {
    f
}

fn accept_fn_once<Args, F: EmulatedFnOnce<Args>>(f: F) -> F {
    f
}
