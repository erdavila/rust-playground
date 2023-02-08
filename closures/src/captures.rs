use crate::Type;

pub fn captures() {
    access();
    mutate();
    use_ownership();
}

fn access() {
    let mut value = Type;
    let closure = || value.access();

    let closure = accept_fn(closure);
    let closure = accept_fn_mut(closure);
    let closure = accept_fn_once(closure);

    value.access();
    // value.mutate(); // Cannot borrow as mutable because it is borrowed as immutable by the closure

    closure();
    closure(); // Can be called again

    value.mutate(); // Can borrow as mutable because it is not borrowed anymore
}

fn mutate() {
    let mut value = Type;
    let closure = || value.mutate();

    // let closure = accept_fn(closure); // Closure that mutates captured value cannot be Fn
    let closure = accept_fn_mut(closure);
    let closure = accept_fn_once(closure);

    // value.access(); // Cannot borrow as immutable because it is borrowed as mutable by the closure
    // value.mutate(); // Cannot borrow as mutable because it is borrowed as mutable by the closure

    let mut closure = closure;
    closure();
    closure(); // Can be called again

    value.mutate(); // Can borrow as mutable because it is not borrowed anymore
}

fn use_ownership() {
    let value = Type;
    let closure = || value.use_ownership();

    // let closure = accept_fn(closure); // Closure that uses ownership of captured value cannot be Fn
    // let closure = accept_fn_mut(closure); // Closure that uses ownership of captured value cannot be FnMut
    let closure = accept_fn_once(closure);

    // value.access(); // Cannot borrow moved value
    // value.mutate(); // Cannot borrow moved value

    closure();
    // closure(); // Cannot be called again

    // value.mutate(); // Cannot borrow moved value
}

fn accept_fn<F: Fn()>(f: F) -> F {
    f
}

fn accept_fn_mut<F: FnMut()>(f: F) -> F {
    f
}

fn accept_fn_once<F: FnOnce()>(f: F) -> F {
    f
}
