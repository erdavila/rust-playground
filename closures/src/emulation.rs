use crate::Type;

pub fn emulation() {
    emulate_fn();
    emulate_fn_mut();
    emulate_fn_once();
}

fn emulate_fn() {
    let mut value = Type;

    struct Closure<'a> {
        captured: &'a Type,
    }
    impl FnOnceClosure<()> for Closure<'_> {
        type Output = ();
        fn call_once(mut self, args: ()) -> Self::Output {
            self.call_mut(args)
        }
    }
    impl FnMutClosure<()> for Closure<'_> {
        fn call_mut(&mut self, args: ()) -> Self::Output {
            self.call(args);
        }
    }
    impl FnClosure<()> for Closure<'_> {
        fn call(&self, _: ()) -> Self::Output {
            self.captured.access();
        }
    }
    let closure = Closure { captured: &value };

    let closure = accept_fn(closure);
    let closure = accept_fn_mut(closure);
    let closure = accept_fn_once(closure);

    value.access();
    // value.mutate(); // Cannot borrow as mutable because it is borrowed as immutable by the closure

    closure.call(());
    closure.call(()); // Can be called again

    value.mutate(); // Can borrow as mutable because it is not borrowed anymore
}

fn emulate_fn_mut() {
    let mut value = Type;

    struct Closure<'a> {
        captured: &'a mut Type,
    }
    impl FnOnceClosure<()> for Closure<'_> {
        type Output = ();
        fn call_once(mut self, args: ()) -> Self::Output {
            self.call_mut(args)
        }
    }
    impl FnMutClosure<()> for Closure<'_> {
        fn call_mut(&mut self, _: ()) -> Self::Output {
            self.captured.mutate();
        }
    }
    let closure = Closure {
        captured: &mut value,
    };

    // let closure = accept_fn(closure); // Closure that mutates captured value cannot be FnClosure
    let closure = accept_fn_mut(closure);
    let closure = accept_fn_once(closure);

    // value.access(); // Cannot borrow as immutable because it is borrowed as mutable by the closure
    // value.mutate(); // Cannot borrow as mutable because it is borrowed as mutable by the closure

    let mut closure = closure;
    closure.call_mut(());
    closure.call_mut(()); // Can be called again

    value.mutate(); // Can borrow as mutable because it is not borrowed anymore
}

fn emulate_fn_once() {
    let value = Type;

    struct Closure {
        captured: Type,
    }
    impl FnOnceClosure<()> for Closure {
        type Output = ();
        fn call_once(self, _: ()) -> Self::Output {
            self.captured.use_ownership();
        }
    }
    let closure = Closure { captured: value };

    // let closure = accept_fn(closure); // Closure that uses ownership of captured value cannot be FnClosure
    // let closure = accept_fn_mut(closure); // Closure that uses ownership of captured value cannot be FnMutClosure
    let closure = accept_fn_once(closure);

    // value.access(); // Cannot borrow moved value
    // value.mutate(); // Cannot borrow moved value

    closure.call_once(());
    // closure.call_once(()); // Cannot be called again

    // value.mutate(); // Cannot borrow moved value
}

trait FnOnceClosure<Args> {
    type Output;

    fn call_once(self, args: Args) -> Self::Output;
}

trait FnMutClosure<Args>: FnOnceClosure<Args> {
    fn call_mut(&mut self, args: Args) -> Self::Output;
}

trait FnClosure<Args>: FnMutClosure<Args> {
    fn call(&self, args: Args) -> Self::Output;
}

fn accept_fn<Args, F: FnClosure<Args>>(f: F) -> F {
    f
}

fn accept_fn_mut<Args, F: FnMutClosure<Args>>(f: F) -> F {
    f
}

fn accept_fn_once<Args, F: FnOnceClosure<Args>>(f: F) -> F {
    f
}
