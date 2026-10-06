#![feature(generic_const_exprs, macro_metavar_expr)]

use symbolic_calculus::util::{FixedType, Sqr as _};
use symbolic_calculus::{
    Function, NamedConstId, StaticZero, TypedConst, Variable, define_named_const,
};

fn main() {
    define_named_const!(x0: X0);
    define_named_const!(v0: V0);
    define_named_const!(a: A);

    let t = Variable::with_name0("t");
    let two = TypedConst::new(2.0);

    let x = x0 + v0 * t + a * t.sqr() / two;
    println!("x = {x}");

    let x_deriv = x.derivative::<0>();
    println!("x' = {x_deriv}");

    let x_deriv_deriv = x_deriv.derivative::<0>();
    println!("x\" = {x_deriv_deriv}");

    println!();
    println!("Setting consts");
    let x = x
        .set_named_const_value::<X0, _>(TypedConst::new(10.0))
        .set_named_const_value::<V0, _>(TypedConst::new(5.0))
        .set_named_const_value::<A, _>(StaticZero::<FixedType<f64>>::new());
    println!("x = {x}");

    let arg = 1.0;
    println!("x({}) = {}", arg, x.apply(arg));
}
