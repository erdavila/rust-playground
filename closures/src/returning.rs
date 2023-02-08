use crate::Type;

pub fn returning() {
    get_no_capture_fn()();
    get_no_capture_fn_mut()();
    get_no_capture_fn_once()();

    get_access_fn()();
    get_access_fn_mut()();
    get_access_fn_once()();

    // get_mutate_fn()();
    get_mutate_fn_mut()();
    get_mutate_fn_once()();

    // get_use_ownership_fn()();
    // get_use_ownership_fn_mut()();
    get_use_ownership_fn_once()();
}

fn get_no_capture_fn() -> impl Fn() {
    || {} // No need to move
}

fn get_no_capture_fn_mut() -> impl FnMut() {
    || {} // No need to move
}

fn get_no_capture_fn_once() -> impl FnOnce() {
    || {} // No need to move
}

fn get_access_fn() -> impl Fn() {
    let value = Type;
    move || value.access()
}

fn get_access_fn_mut() -> impl FnMut() {
    let value = Type;
    move || value.access()
}

fn get_access_fn_once() -> impl FnOnce() {
    let value = Type;
    move || value.access()
}

// Closure that mutates captured value cannot be Fn
// fn get_mutate_fn() -> impl Fn() {
//     let mut value = Type;
//     move || value.mutate()
// }

fn get_mutate_fn_mut() -> impl FnMut() {
    let mut value = Type;
    move || value.mutate()
}

fn get_mutate_fn_once() -> impl FnOnce() {
    let mut value = Type;
    move || value.mutate()
}

// Closure that uses ownership of captured value cannot be Fn
// fn get_use_ownership_fn() -> impl Fn() {
//     let value = Type;
//     move || value.use_ownership()
// }

// Closure that uses ownership of captured value cannot be FnMut
// fn get_use_ownership_fn_mut() -> impl FnMut() {
//     let value = Type;
//     move || value.use_ownership()
// }

fn get_use_ownership_fn_once() -> impl FnOnce() {
    let value = Type;
    || value.use_ownership() // No need to move
}
