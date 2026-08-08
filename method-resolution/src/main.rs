/*
    References:
        - https://doc.rust-lang.org/nomicon/dot-operator.html
        - https://doc.rust-lang.org/reference/expressions/method-call-expr.html
*/

mod autoref_specialization;
mod method_resolution;

fn main() {
    autoref_specialization::demo();
    method_resolution::demo();
}
