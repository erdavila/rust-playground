use core::fmt;
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Neg, Sub};

use ops_traits_macros::{ops_bounds, ops_type};
use symbolic_calculus::util::Sqr;
use symbolic_calculus::{
    Apply, Derivative, EvaluatedDisplayData, Function, FunctionDisplay, Output, Squared,
};
use symbolic_calculus_macros::{
    Arity, ComparisonTokens, Display, Function, RenameVariable, SetNamedConstValue, VariableNames,
};

use crate::complex::{Complex, RealWrapper};
use crate::modulo_squared::ModSqr;
use crate::real::Real;
use crate::realification::complex_real_function::{
    AsComplex as _, ComplexRealFunction, FromComplex, IntoComplexRealFunction,
};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Function,
    RenameVariable,
    VariableNames,
    SetNamedConstValue,
    Arity,
    ComparisonTokens,
    Display,
)]
#[function(operands = 1)]
pub(crate) struct Real1Function<F>(pub F);

impl<F> Real1Function<F> {
    fn new(func: F) -> Self {
        Real1Function(func)
    }
}

impl<F, R: Real> Output<R> for Real1Function<F>
where
    F: Output<R>,
{
    type Output = F::Output;
}
impl<F, R: Real> Apply<R> for Real1Function<F>
where
    F: Apply<R>,
{
    fn apply(this: &Self, input: R) -> Self::Output {
        this.0.apply(input)
    }
}

impl<F, R> Output<(R,)> for Real1Function<F>
where
    F: Output<R>,
{
    type Output = (F::Output,);
}
impl<F, R> Apply<(R,)> for Real1Function<F>
where
    F: Apply<R>,
{
    fn apply(this: &Self, (input,): (R,)) -> Self::Output {
        (this.0.apply(input),)
    }
}

impl<F, R> Output<(R, R)> for Real1Function<F>
where
    F: Output<(R, R)>,
{
    type Output = F::Output;
}
impl<F, R> Apply<(R, R)> for Real1Function<F>
where
    F: Apply<(R, R)>,
{
    fn apply(this: &Self, input: (R, R)) -> Self::Output {
        this.0.apply(input)
    }
}

impl<F, R> Output<Complex<R>> for Real1Function<F>
where
    F: Output<(R, R)>,
{
    type Output = F::Output;
}
impl<F, R> Apply<Complex<R>> for Real1Function<F>
where
    F: Apply<(R, R)>,
{
    fn apply(this: &Self, input: Complex<R>) -> Self::Output {
        this.0.apply(input.into_tuple())
    }
}

impl<F> FunctionDisplay for Real1Function<F>
where
    F: FunctionDisplay,
{
    const PRECEDENCE: u8 = u8::MAX;

    fn evaluated_display_data(&self) -> EvaluatedDisplayData<impl Display> {
        let func = self.0.evaluated_display_data();

        let always_const = func.always_const;
        let func_display = func.display_data.display;

        Self::maybe_const_evaluated_display_data(
            always_const,
            fmt::from_fn(move |f| write!(f, "⟨{func_display}⟩")),
        )
    }
}

pub(crate) trait Gradient<I>: Function {
    type Gradient;

    fn gradient(&self) -> Self::Gradient;
}
impl<F, R: Real> Gradient<R> for Real1Function<F>
where
    F: Derivative<0>,
{
    type Gradient = Real1Function<F::Derivative>;

    fn gradient(&self) -> Self::Gradient {
        Real1Function(self.0.derivative())
    }
}
impl<F, R> Gradient<Complex<R>> for Real1Function<F>
where
    F: Derivative<0> + Derivative<1>,
{
    type Gradient =
        ComplexRealFunction<<F as Derivative<0>>::Derivative, <F as Derivative<1>>::Derivative>;

    fn gradient(&self) -> Self::Gradient {
        ComplexRealFunction {
            real: self.0.derivative::<0>(),
            imag: self.0.derivative::<1>(),
        }
    }
}

// -Real1Function<F>
#[ops_bounds(-F)]
impl<F> Neg for Real1Function<F> {
    type Output = Real1Function<ops_type!(-F)>;

    fn neg(self) -> Self::Output {
        Real1Function(-self.0)
    }
}

// Real1Function<F> + Real1Function<F2>
#[ops_bounds(F + F2)]
impl<F, F2> Add<Real1Function<F2>> for Real1Function<F> {
    type Output = Real1Function<ops_type!(F + F2)>;

    fn add(self, rhs: Real1Function<F2>) -> Self::Output {
        Real1Function(self.0 + rhs.0)
    }
}

// Real1Function<F> + ComplexRealFunction<R, I>
#[ops_bounds(RealWrapper<F> + Complex<R, I>: IntoComplexRealFunction)]
impl<F, R, I> Add<ComplexRealFunction<R, I>> for Real1Function<F> {
    type Output = FromComplex<ops_type!(RealWrapper<F> + Complex<R, I>)>;

    fn add(self, rhs: ComplexRealFunction<R, I>) -> Self::Output {
        let l = RealWrapper(self.0);
        rhs.as_complex(|r| l + r)
    }
}

// Real1Function<F> - Real1Function<F2>
#[ops_bounds(F - F2)]
impl<F, F2> Sub<Real1Function<F2>> for Real1Function<F> {
    type Output = Real1Function<ops_type!(F - F2)>;

    fn sub(self, rhs: Real1Function<F2>) -> Self::Output {
        Real1Function(self.0 - rhs.0)
    }
}

// Real1Function<F> - ComplexRealFunction<R, I>
#[ops_bounds(RealWrapper<F> - Complex<R, I>: IntoComplexRealFunction)]
impl<F, R, I> Sub<ComplexRealFunction<R, I>> for Real1Function<F> {
    type Output = FromComplex<ops_type!(RealWrapper<F> - Complex<R, I>)>;

    fn sub(self, rhs: ComplexRealFunction<R, I>) -> Self::Output {
        let l = RealWrapper(self.0);
        rhs.as_complex(|r| l - r)
    }
}

// Real1Function<F> * Real1Function<F2>
#[ops_bounds(F * F2)]
impl<F, F2> Mul<Real1Function<F2>> for Real1Function<F> {
    type Output = Real1Function<ops_type!(F * F2)>;

    fn mul(self, rhs: Real1Function<F2>) -> Self::Output {
        Real1Function(self.0 * rhs.0)
    }
}

// Real1Function<F> * ComplexRealFunction<R, I>
#[ops_bounds(RealWrapper<F> * Complex<R, I>: IntoComplexRealFunction)]
impl<F, R, I> Mul<ComplexRealFunction<R, I>> for Real1Function<F> {
    type Output = FromComplex<ops_type!(RealWrapper<F> * Complex<R, I>)>;

    fn mul(self, rhs: ComplexRealFunction<R, I>) -> Self::Output {
        let l = RealWrapper(self.0);
        rhs.as_complex(|r| l * r)
    }
}

// Real1Function<F> / Real1Function<G>
#[ops_bounds(F / G)]
impl<F, G> Div<Real1Function<G>> for Real1Function<F> {
    type Output = Real1Function<ops_type!(F / G)>;

    fn div(self, rhs: Real1Function<G>) -> Self::Output {
        Real1Function(self.0 / rhs.0)
    }
}

// Real1Function<F> / ComplexRealFunction<R, I>
#[ops_bounds(RealWrapper<F> / Complex<R, I>: IntoComplexRealFunction)]
impl<F, R, I> Div<ComplexRealFunction<R, I>> for Real1Function<F> {
    type Output = FromComplex<ops_type!(RealWrapper<F> / Complex<R, I>)>;

    fn div(self, rhs: ComplexRealFunction<R, I>) -> Self::Output {
        let l = RealWrapper(self.0);
        rhs.as_complex(|r| l / r)
    }
}

// Real1Function<F>²
#[ops_bounds(F ^ 2)]
impl<F> Sqr for Real1Function<F> {
    type Output = Real1Function<ops_type!(F ^ 2)>;

    fn sqr(self) -> Self::Output {
        Real1Function(self.0.sqr())
    }
}

// |Real1Function<F>|²
// TODO: can it be implemented without referencing the `Squared` function directly?
impl<F> ModSqr for Real1Function<F> {
    type Output = Real1Function<Squared<F>>;

    fn mod_sqr(self) -> Self::Output {
        Real1Function(Squared(self.0))
    }
}
