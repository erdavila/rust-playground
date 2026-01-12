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

                // obj_mut_ref.mutate(); // Mutability required
                // owned.mutate(); // Mutability required

                // owned.r#move(); // Ownership required
            }
        }
        EmulatedFn(
            obj_ref,     // Immutable reference captured as immutable reference
            obj_mut_ref, // Mutable reference captured as immutable reference
            &owned,      // Owned value captured as immutable reference
        )
    };

    let closure = accept_fn(closure);
    let closure = accept_fn_mut(closure);
    let closure = accept_fn_once(closure);

    obj_ref.access();
    obj_mut_ref.access();
    owned.access();

    // obj_mut_ref.mutate(); // Cannot borrow as mutable because it is borrowed as mutable by the closure
    // owned.mutate(); // Cannot borrow as mutable because it is borrowed as immutable by the closure

    // owned.r#move(); // Cannot move because it is borrowed by the closure

    closure.call(());
    closure.call(()); // Can be called again

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

                // owned.r#move(); // Cannot move because it is borrowed
            }
        }
        EmulatedFnMut(
            obj_ref,     // Immutable reference captured as immutable reference
            obj_mut_ref, // Mutable reference captured as mutable reference
            &mut owned,  // Owned value captured as mutable reference
        )
    };

    // let closure = accept_fn(closure); // Closure that mutates captured value cannot be FnClosure
    let closure = accept_fn_mut(closure);
    let closure = accept_fn_once(closure);

    obj_ref.access();
    // obj_mut_ref.access(); // Cannot borrow as immutable because it is borrowed as mutable by the closure
    // owned.access(); // Cannot borrow as immutable because it is borrowed as mutable by the closure

    // obj_mut_ref.mutate(); // Cannot borrow as mutable because it is borrowed as mutable by the closure
    // owned.mutate(); // Cannot borrow as mutable because it is borrowed as immutable by the closure

    // owned.r#move(); // Cannot move because it is borrowed by the closure

    let mut closure = closure;
    closure.call_mut(());
    closure.call_mut(()); // Can be called again

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
    let owned = Object; // EMULATION DISCREPANCY: Mutability is not required

    let closure = {
        struct EmulatedFnOnce<'a>(&'a Object, &'a mut Object, Object);
        impl self::EmulatedFnOnce<()> for EmulatedFnOnce<'_> {
            type Output = ();
            fn call_once(self, (): ()) -> Self::Output {
                let EmulatedFnOnce(
                    obj_ref,
                    obj_mut_ref,
                    mut owned, // EMULATION: Mutability must be signaled here
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
            obj_ref,     // Immutable reference captured as immutable reference
            obj_mut_ref, // Mutable reference captured as mutable reference
            owned,       // Owned value captured by moving
        )
    };

    // let closure = accept_fn(closure); // Closure that uses ownership of captured value cannot be FnClosure
    // let closure = accept_fn_mut(closure); // Closure that uses ownership of captured value cannot be FnMutClosure
    let closure = accept_fn_once(closure);

    obj_ref.access();
    // obj_mut_ref.access(); // Cannot borrow as immutable because it is borrowed as mutable by the closure
    // owned.access(); // Cannot borrow because it is moved by the closure

    // obj_mut_ref.mutate(); // Cannot borrow as mutable because it is borrowed as mutable by the closure
    // owned.mutate(); // Cannot borrow because it is moved by the closure

    // owned.r#move(); // Cannot move because it is moved by the closure

    closure.call_once(());
    // closure.call_once(()); // Cannot be called again

    obj_ref.access();
    obj_mut_ref.access();
    // owned.access(); // Cannot borrow because it was moved by the closure

    obj_mut_ref.mutate();
    // owned.mutate(); // Cannot borrow because it was moved by the closure

    // owned.r#move(); // Cannot borrow because it was moved by the closure
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

                // obj_mut_ref.mutate(); // Cannot borrow as mutable because it is borrowed as immutable
                // owned.mutate(); // Cannot borrow as mutable because it is borrowed as immutable

                // owned.r#move(); // Cannot move because it is borrowed
            }
        }
        EmulatedMovingFn(
            obj_ref,     // Immutable reference captured as immutable reference
            obj_mut_ref, // Immutable reference captured as immutable reference
            owned,       // Owned value captured by moving
        )
    };

    let closure = accept_fn(closure);
    let closure = accept_fn_mut(closure);
    let closure = accept_fn_once(closure);

    obj_ref.access();
    // obj_mut_ref.access(); // EMULATION DISCREPANCY: mutable reference should have been moved into closure
    // owned.access(); // Moved into closure

    closure.call(());
    closure.call(()); // Can be called again

    obj_ref.access();
    // obj_mut_ref.access(); // EMULATION DISCREPANCY: mutable reference should have been moved into closure
    // owned.access(); // Moved into closure
}

fn test_moving_fn_mut() {
    let obj_ref = &Object;
    let obj_mut_ref = &mut Object;
    let owned = Object; // EMULATION DISCREPANCY: Mutability is not required

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

                // owned.r#move(); // Cannot move because it is borrowed
            }
        }
        EmulatedMovingFnMut(
            obj_ref,     // Immutable reference captured as immutable reference
            obj_mut_ref, // Mutable reference captured as mutable reference
            owned,       // Owned value captured by moving
        )
    };

    // let closure = accept_fn(closure); // Closure that mutates captured value cannot be FnClosure
    let closure = accept_fn_mut(closure);
    let closure = accept_fn_once(closure);

    obj_ref.access();
    // obj_mut_ref.access();  // Moved into closure
    // owned.access(); // Moved into closure

    let mut closure = closure;
    closure.call_mut(());
    closure.call_mut(()); // Can be called again

    obj_ref.access();
    // obj_mut_ref.access(); // EMULATION DISCREPANCY: mutable reference should have been moved into closure
    // owned.access(); // Moved into closure
}

fn test_moving_fn_once() {
    let obj_ref = &Object;
    let obj_mut_ref = &mut Object;
    let owned = Object; // EMULATION DISCREPANCY: Mutability is not required

    let closure = {
        struct EmulatedMovingFnOnce<'a>(&'a Object, &'a mut Object, Object);
        impl self::EmulatedFnOnce<()> for EmulatedMovingFnOnce<'_> {
            type Output = ();
            fn call_once(self, (): ()) -> Self::Output {
                let EmulatedMovingFnOnce(
                    obj_ref,
                    obj_mut_ref,
                    mut owned, // EMULATION: Mutability must be signaled here
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
            obj_ref,     // Immutable reference captured as immutable reference
            obj_mut_ref, // Mutable reference captured as mutable reference
            owned,       // Owned value captured by moving
        )
    };

    // let closure = accept_fn(closure); // Closure that uses ownership of captured value cannot be FnClosure
    // let closure = accept_fn_mut(closure); // Closure that uses ownership of captured value cannot be FnMutClosure
    let closure = accept_fn_once(closure);

    obj_ref.access();
    // obj_mut_ref.access(); // Moved into closure
    // owned.access(); // Moved into closure

    closure.call_once(());
    // closure.call_once(()); // Cannot be called again

    obj_ref.access();
    // obj_mut_ref.access(); // EMULATION DISCREPANCY: mutable reference should have been moved into closure
    // owned.access(); // Moved into closure
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
