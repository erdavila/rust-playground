struct Object;
#[expect(clippy::unused_self)]
impl Object {
    fn access(&self) {}
    fn mutate(&mut self) {}
    fn r#move(self) {}
}

mod captures;
mod emulation;
mod moved_captures;

fn main() {
    captures::test();
    moved_captures::test();
    emulation::test();
    println!("OK");
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
