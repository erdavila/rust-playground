use crate::{Object, accept_fn, accept_fn_mut, accept_fn_once};

pub fn test() {
    test_fn();
    test_fn_mut();
    test_fn_once();
}

fn test_fn() {
    let obj_ref = &Object;
    let obj_mut_ref = &mut Object;
    let owned = Object;

    let closure = move || {
        obj_ref.access();
        obj_mut_ref.access();
        owned.access();

        // Mutability required - the closure couldn't be Fn.
        // obj_mut_ref.mutate();
        // Mutability required - the closure couldn't be Fn.
        // owned.mutate();

        // Ownership required - the closure couldn't be Fn.
        // owned.r#move();
    };

    let closure = accept_fn(closure);
    let closure = accept_fn_mut(closure);
    let closure = accept_fn_once(closure);

    obj_ref.access();
    // Moved into closure.
    // obj_mut_ref.access();
    // Moved into closure.
    // owned.access();

    closure();
    closure();

    obj_ref.access();
    // obj_mut_ref.access(); // Moved into closure.
    // owned.access(); // Moved into closure.
}

fn test_fn_mut() {
    let obj_ref = &Object;
    let obj_mut_ref = &mut Object;
    let mut owned = Object;

    let closure = move || {
        obj_ref.access();
        obj_mut_ref.access();
        owned.access();

        obj_mut_ref.mutate();
        owned.mutate();

        // Ownership required - the closure couldn't be FnMut.
        // owned.r#move();
    };

    // Closure that mutates captured value cannot be Fn.
    // let closure = accept_fn(closure);
    let closure = accept_fn_mut(closure);
    let closure = accept_fn_once(closure);
    let mut closure = closure;

    obj_ref.access();
    // Moved into closure.
    // obj_mut_ref.access();
    // Moved into closure.
    // owned.access();

    closure();
    closure();

    obj_ref.access();
    // Moved into closure.
    // obj_mut_ref.access();
    // Moved into closure.
    // owned.access();
}

fn test_fn_once() {
    let obj_ref = &Object;
    let obj_mut_ref = &mut Object;
    let mut owned = Object;

    let closure = move || {
        obj_ref.access();
        obj_mut_ref.access();
        owned.access();

        obj_mut_ref.mutate();
        owned.mutate();

        owned.r#move();
    };

    // Closure that uses ownership of captured value cannot be Fn.
    // let closure = accept_fn(closure);
    // Closure that uses ownership of captured value cannot be Fn.
    // let closure = accept_fn_mut(closure);
    let closure = accept_fn_once(closure);

    obj_ref.access();
    // Moved into closure.
    // obj_mut_ref.access();
    // Moved into closure.
    // owned.access();

    closure();
    // Cannot be called again.
    // closure();

    obj_ref.access();
    // Moved into closure.
    // obj_mut_ref.access();
    // Moved into closure.
    // owned.access();
}
