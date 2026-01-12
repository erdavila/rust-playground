use emulated_closures::{
    composition::Compose as _, emulated_fn, emulated_fn_mut, emulated_fn_once, EmulatedFn as _,
    EmulatedFnMut as _, EmulatedFnOnce as _,
};

#[test]
fn test_emulated_fn_once() {
    let f = emulated_fn_once! {
        captures: {},
        signature: (String,) => usize,
        |(), (s,)| {
            s.len()
        }
    };

    let g = emulated_fn_once! {
        captures: {},
        signature: (char,) => String,
        |(), (c,)| {
            String::from(c)
        }
    };

    let composed = f.compose(g);

    let result = composed.call_once(('@',));

    assert_eq!(result, 1);
}

#[test]
fn test_emulated_fn_mut() {
    let f = emulated_fn_mut! {
        captures: {},
        signature: (String,) => usize,
        |(), (s,)| {
            s.len()
        }
    };

    let g = emulated_fn_mut! {
        captures: {},
        signature: (char,) => String,
        |(), (c,)| {
            String::from(c)
        }
    };

    let mut composed = f.compose(g);

    let result = composed.call_mut(('@',));

    assert_eq!(result, 1);
}

#[test]
fn test_emulated_fn() {
    let f = emulated_fn! {
        captures: {},
        signature: (String,) => usize,
        |(), (s,)| {
            s.len()
        }
    };

    let g = emulated_fn! {
        captures: {},
        signature: (char,) => String,
        |(), (c,)| {
            String::from(c)
        }
    };

    let composed = f.compose(g);

    let result = composed.call(('@',));

    assert_eq!(result, 1);
}
