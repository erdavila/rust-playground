use fn_bind::Bind2;

fn main() {
    as_fn(find).bind1("Hello, world!")("ll");
    // as_fn(find).bind1_clone("Hello, world!")("ll");
    as_fn(find).bind2("Hello, world!", "ll")();
    // as_fn(find).bind2_clone("Hello, world!", "ll")();

    as_fn_clone(find).bind1("Hello, world!")("ll");
    as_fn_clone(find).bind1_clone("Hello, world!")("ll");
    as_fn_clone(find).bind2("Hello, world!", "ll")();
    as_fn_clone(find).bind2_clone("Hello, world!", "ll")();
}

fn find<'a>(needle: &str, haystack: &'a str) -> Option<&'a str> {
    haystack
        .find(needle)
        .map(|index| &haystack[index..index + needle.len()])
}

fn as_fn<A1, A2, R>(f: impl Fn(A1, A2) -> R) -> impl Fn(A1, A2) -> R {
    f
}
fn as_fn_clone<A1, A2, R>(f: impl Fn(A1, A2) -> R + Clone) -> impl Fn(A1, A2) -> R + Clone {
    f
}
