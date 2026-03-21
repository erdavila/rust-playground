#![feature(unboxed_closures)]

trait Bind1<A>: FnOnce<(A,)> {
    fn bind1(self, a: A) -> impl FnOnce<(), Output = Self::Output>
    where
        Self: Sized,
    {
        || self(a)
    }

    fn bind1_clone(self, a: A) -> impl Fn<(), Output = Self::Output> + Clone
    where
        Self: Clone,
        A: Clone,
    {
        move || {
            let f = self.clone();
            let a = a.clone();
            f(a)
        }
    }
}
impl<F, A> Bind1<A> for F where F: FnOnce<(A,)> {}

trait Bind2<A1, A2>: FnOnce<(A1, A2)> {
    fn bind1(self, a1: A1) -> impl FnOnce<(A2,), Output = Self::Output>
    where
        Self: Sized,
    {
        |a2| self(a1, a2)
    }

    fn bind1_clone(self, a1: A1) -> impl Fn<(A2,), Output = Self::Output> + Clone
    where
        Self: Clone,
        A1: Clone,
    {
        move |a2| {
            let f = self.clone();
            let a1 = a1.clone();
            f(a1, a2)
        }
    }

    fn bind2_once(self, a1: A1, a2: A2) -> impl FnOnce<(), Output = Self::Output>
    where
        Self: Sized,
    {
        self.bind1(a1).bind1(a2)
    }

    fn bind2_clone(self, a1: A1, a2: A2) -> impl Fn<(), Output = Self::Output> + Clone
    where
        Self: Clone,
        A1: Clone,
        A2: Clone,
    {
        self.bind1_clone(a1).bind1_clone(a2)
    }
}
impl<F, A1, A2> Bind2<A1, A2> for F where F: FnOnce<(A1, A2)> {}

fn main() {
    as_fn(find).bind1("Hello, world!")("ll");
    // as_fn(find).bind1_clone("Hello, world!")("ll");
    as_fn(find).bind2_once("Hello, world!", "ll")();
    // as_fn(find).bind2_clone("Hello, world!", "ll")();

    as_fn_clone(find).bind1("Hello, world!")("ll");
    as_fn_clone(find).bind1_clone("Hello, world!")("ll");
    as_fn_clone(find).bind2_once("Hello, world!", "ll")();
    as_fn_clone(find).bind2_clone("Hello, world!", "ll")();
}

fn find<'a>(needle: &str, haystack: &'a str) -> Option<&'a str> {
    haystack
        .find(needle)
        .map(|index| &haystack[index..index + needle.len()])
}

fn as_fn<A1, A2>(f: impl Fn<(A1, A2)>) -> impl Fn<(A1, A2)> {
    f
}
fn as_fn_clone<A1, A2>(f: impl Fn<(A1, A2)> + Clone) -> impl Fn<(A1, A2)> + Clone {
    f
}
