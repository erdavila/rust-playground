use crate::{Object, accept_fn, accept_fn_mut, accept_fn_once};

pub fn test() {
    test_fn();
    test_fn_mut();
    test_fn_once();
}

fn test_fn() {
    let obj_ref = &Object;
    let obj_mut_ref = &mut Object;
    let mut owned = Object;

    let closure = || {
        obj_ref.access();
        obj_mut_ref.access();
        owned.access();

        // obj_mut_ref.mutate(); // Mutability required - the closure couldn't be Fn
        // owned.mutate(); // Mutability required - the closure couldn't be Fn

        // owned.r#move(); // Ownership required - the closure couldn't be Fn
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

    closure();
    closure(); // Can be called again

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

    let closure = || {
        obj_ref.access();
        obj_mut_ref.access();
        owned.access();

        obj_mut_ref.mutate();
        owned.mutate();

        // owned.r#move(); // Ownership required - the closure couldn't be FnMut
    };

    // let closure = accept_fn(closure); // Closure that mutates captured value cannot be Fn
    let closure = accept_fn_mut(closure);
    let closure = accept_fn_once(closure);
    let mut closure = closure;

    obj_ref.access();
    // obj_mut_ref.access(); // Cannot borrow as immutable because it is borrowed as mutable by the closure
    // owned.access(); // Cannot borrow as immutable because it is borrowed as mutable by the closure

    // obj_mut_ref.mutate(); // Cannot borrow as mutable because it is borrowed as mutable by the closure
    // owned.mutate(); // Cannot borrow as mutable because it is borrowed as mutable by the closure

    // owned.r#move(); // Cannot move because it is borrowed by the closure

    closure();
    closure(); // Can be called again

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
    let mut owned = Object;

    let closure = || {
        obj_ref.access();
        obj_mut_ref.access();
        owned.access();

        obj_mut_ref.mutate();
        owned.mutate();

        owned.r#move();
    };

    // let closure = accept_fn(closure); // Closure that uses ownership of captured value cannot be Fn
    // let closure = accept_fn_mut(closure); // Closure that uses ownership of captured value cannot be Fn
    let closure = accept_fn_once(closure);

    obj_ref.access();
    // obj_mut_ref.access(); // Cannot borrow as immutable because it is borrowed as mutable by the closure
    // owned.access(); // Cannot borrow because it is moved by the closure

    // obj_mut_ref.mutate(); // Cannot borrow as mutable because it is borrowed as mutable by the closure
    // owned.mutate(); // Cannot borrow because it is moved by the closure

    // owned.r#move(); // Cannot move because it is moved by the closure

    closure();
    // closure(); // Cannot be called again

    obj_ref.access();
    obj_mut_ref.access();
    // owned.access(); // Cannot borrow because it was moved by the closure

    obj_mut_ref.mutate();
    // owned.mutate(); // Cannot borrow because it was moved by the closure

    // owned.r#move(); // Cannot borrow because it was moved by the closure
}
