#![expect(
    clippy::needless_borrow,
    clippy::trivially_copy_pass_by_ref,
    clippy::unused_self
)]

use std::ops::Deref;

trait Trait {
    fn trait_by_reference(&self) {}
}

struct A(B);

impl A {
    fn a_by_value(self) {}
    fn a_by_reference(&self) {}
}

impl Trait for &&A {}

impl Deref for A {
    type Target = B;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Copy)]
struct B;

impl B {
    fn b_by_value(self) {}
    fn b_by_reference(&self) {}
}

pub(super) fn demo() {
    a_by_value();
    a_by_reference();

    b_by_value();
    b_by_reference();

    trait_by_reference();
}

fn a_by_value() {
    A(B).a_by_value();
    /*
        1. By value: A::a_by_value(A)
            Found A::a_by_value(sekf)
            Executes:
                let receiver: A = A(B);
                A::a_by_value(receiver);
    */

    // (&A(B)).a_by_value();
    /*
        1. By value: <&A>::a_by_value(&A)
            Not found.
        2. Autoref: <&A>::a_by_value(&<&A>)
            Not found.
        3. Deref: <&A as Deref>::Target = A
            1. By value: A::a_by_value(A)
                Found A::a_by_value(self)
                Executes:
                    let receiver: &A = &A(B);
                    let a_ref: &A = <&A as Deref>::deref(&receiver);
                    A::a_by_value(*a_ref);
                ERROR:
                    cannot move out of a shared reference
                    move occurs because value has type `A`, which does not implement the `Copy` trait
    */
}

fn a_by_reference() {
    A(B).a_by_reference();
    /*
        1. By value: A::a_by_reference(A)
            Not found.
        2. Autoref: A::a_by_reference(&A)
            Found A::a_by_reference(&self)
            Executes:
                let receiver: A = A(B);
                A::a_by_reference(&receiver);
    */

    (&A(B)).a_by_reference();
    /*
        1. By value: <&A>::a_by_reference(&A)
            Not found.
        2. Autoref: <&A>::a_by_reference(&<&A>)
            Not found.
        3. Deref: <&A as Deref>::Target = A
            1. By value: A::a_by_reference(A)
                Not found.
            2. Autoref: A::a_by_reference(&A)
                Found A::a_by_reference(&self)
                Executes:
                    let receiver: &A = &A(B);
                    let a_ref: &A = <&A as Deref>::deref(&receiver);
                    A::a_by_reference(&*a_ref);
    */

    (&&A(B)).a_by_reference();
    /*
        1. By value: <&&A>::a_by_reference(&&A)
            Not found.
        2. Autoref: <&&A>::a_by_reference(&<&&A>)
            Not found.
        3. Deref: <&&A as Deref>::Target = &A
            1. By value: <&A>::a_by_reference(&A)
                Not found.
            2. Autoref: <&A>::a_by_reference(&<&A>)
                Not found.
            3. Deref: <&A as Deref>::Target = A
                1. By value: A::a_by_reference(A)
                    Not found.
                2. Autoref: A::a_by_reference(&A)
                    Found A::a_by_reference(&self)
                    Executes:
                        let receiver: &&A = &&A(B);
                        let a_ref_ref: &&A = <&&A as Deref>::deref(&receiver);
                        let a_ref: &A = <&A as Deref>::deref(a_ref_ref);
                        A::a_by_reference(&*a_ref);
    */
}

fn b_by_value() {
    A(B).b_by_value();
    /*
        1. By value: A::b_by_value(A)
            Not found.
        2. Autoref: A::b_by_value(&A)
            Not found.
        3. Deref: <A as Deref>::Target = B
            1. By value: B::b_by_value(B)
                Found B::b_by_value(self)
                Executes:
                    let receiver: A = A(B);
                    let b_ref: &B = <A as Deref>::deref(&receiver);
                    B::b_by_value(*b_ref);
    */

    (&A(B)).b_by_value();
    /*
        1. By value: <&A>::b_by_value(&A)
            Not found.
        2. Autoref: <&A>::b_by_value(&<&A>)
            Not found.
        3. Deref: <&A as Deref>::Target = A
            1. By value: A::b_by_value(A)
                Not found.
            2. Autoref: A::b_by_value(&A)
                Not found.
            3. Deref: <A as Deref>::Target = B
                1. By value: B::b_by_value(B)
                    Found B::b_by_value(self)
                    Executes:
                        let receiver: &A = &A(B);
                        let a_ref: &A = <&A as Deref>::deref(&receiver);
                        let b_ref: &B = <A as Deref>::deref(a_ref);
                        B::b_by_value(*b_ref);
    */
}

fn b_by_reference() {
    A(B).b_by_reference();
    /*
        1. By value: A::b_by_reference(A)
            Not found.
        2. Autoref: A::b_by_reference(&A)
            Not found.
        3. Deref: <A as Deref>::Target = B
            1. By value: B::b_by_reference(B)
                Not found.
            2. Autoref: B::b_by_reference(&B)
                Found B::b_by_reference(&self)
                Executes:
                    let receiver: A = A(B);
                    let b_ref: &B = <A as Deref>::deref(&receiver);
                    B::b_by_reference(&*b_ref);
    */

    (&A(B)).b_by_reference();
    /*
        1. By value: <&A>::b_by_reference(&A)
            Not found.
        2. Autoref: <&A>::b_by_reference(&<&A>)
            Not found.
        3. Deref: <&A as Deref>::Target = A
            1. By value: A::b_by_reference(A)
                Not found.
            2. Autoref: A::b_by_reference(&A)
                Not found.
            3. Deref: <A as Deref>::Target = B
                1. By value: B::b_by_reference(B)
                    Not found.
                2. Autoref: B::b_by_reference(&B)
                    Found B::b_by_reference(&self)
                    Executes:
                        let receiver: &A = &A(B);
                        let a_ref: &A = <&A as Deref>::deref(&receiver);
                        let b_ref: &B = <A as Deref>::deref(a_ref);
                        B::b_by_reference(&*b_ref);
    */
}

fn trait_by_reference() {
    // A(B).trait_by_reference();
    /*
        1. By value: A::trait_by_reference(A)
            Not found.
        2. Autoref: A::trait_by_reference(&A)
            Not found.
        3. Deref: <A as Deref>::Target = B
            1. By value: B::trait_by_reference(B)
                Not found.
            2. Autoref: B::trait_by_reference(&B)
                Not found
            3. Deref: the trait bound `B: Deref` is not satisfied
            ERROR:
                no method named `trait_by_reference` found for struct `A` in the current scope
                items from traits can only be used if the trait is implemented and in scope
    */

    // (&A(B)).trait_by_reference();
    /*
        1. By value: <&A>::trait_by_reference(&A)
            Not found.
        2. Autoref: <&A>::trait_by_reference(&<&A>)
            Not found.
        3. Deref: <&A as Deref>::Target = A
            1. By value: A::trait_by_reference(A)
                Not found.
            2. Autoref: A::trait_by_reference(&A)
                Not found.
            3. Deref: <A as Deref>::Target = B
                1. By value: B::trait_by_reference(B)
                    Not found.
                2. Autoref: B::trait_by_reference(&B)
                    Not found
                3. Deref: the trait bound `B: Deref` is not satisfied
                ERROR:
                    no method named `trait_by_reference` found for reference `&A` in the current scope
                    items from traits can only be used if the trait is implemented and in scope
    */

    (&&A(B)).trait_by_reference();
    /*
        1. By value: <&&A>::trait_by_reference(&&A)
            Not found.
        2. Autoref: <&&A>::trait_by_reference(&<&&A>)
            Found <&&A as Trait>::trait_by_reference(&self)
            Executes: <&&A as Trait>::trait_by_reference(&(&&A(B)));
    */

    (&&&A(B)).trait_by_reference();
    /*
        1. By value: <&&&A>::trait_by_reference(&&&A)
            Not found.
        2. Autoref: <&&&A>::trait_by_reference(&<&&&A>)
            Not found.
        3. Deref: <&&&A as Deref>::Target = &&A
            1. By value: <&&A>::trait_by_reference(&&A)
                Not found.
            2. Autoref: <&&A>::trait_by_reference(&<&&A>)
                Found <&&A as Trait>::trait_by_reference(&self)
                Executes:
                    let a_ref_ref_ref: &&&A = <&&&A as Deref>::deref(&(&&&A(B)));
                    <&&A>::trait_by_reference(&*a_ref_ref_ref);
    */

    (&&&&A(B)).trait_by_reference();
    /*
        1. By value: <&&&&A>::trait_by_reference(&&&&A)
            Not found.
        2. Autoref: <&&&&A>::trait_by_reference(&<&&&&A>)
            Not found.
        3. Deref: <&&&&A as Deref>::Target = &&&A
            1. By value: <&&&A>::trait_by_reference(&&&A)
                Not found.
            2. Autoref: <&&&A>::trait_by_reference(&<&&&A>)
                Not found.
            3. Deref: <&&&A as Deref>::Target = &&A
                1. By value: <&&A>::trait_by_reference(&&A)
                    Not found.
                2. Autoref: <&&A>::trait_by_reference(&<&&A>)
                    Found <&&A as Trait>::trait_by_reference(&self)
                    Executes:
                        let a_ref_ref_ref_ref: &&&&A = <&&&&A as Deref>::deref(&(&&&&A(B)));
                        let a_ref_ref_ref: &&&A = <&&&A as Deref>::deref(a_ref_ref_ref_ref);
                        <&&A>::trait_by_reference(&*a_ref_ref_ref);
    */
}
