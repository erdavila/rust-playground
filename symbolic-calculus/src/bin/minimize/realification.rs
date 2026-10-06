mod complex_real_function;
mod real_1_function;

use std::fmt::Display;

use ops_traits_macros::{ops_bounds, ops_type};
pub(crate) use real_1_function::Gradient;
use symbolic_calculus::util::Sqr;
use symbolic_calculus::{
    Addition, Division, Function, Multiplication, Negation, Output, Squared, StaticOne, StaticTwo,
    StaticZero, Subtraction, TypedConst, Variable,
};

use crate::complex::Complex;
use crate::modulo_squared::{ModSqr, ModuloSquared};
use crate::real::Real;
use crate::realification::complex_real_function::ComplexRealFunction;
use crate::realification::real_1_function::Real1Function;

pub(crate) trait Realify: Function {
    fn realify<I>(&self) -> <Self as Realified<I>>::Realified
    where
        Self: Realified<I>,
    {
        self.realified()
    }
}
impl<F: Function> Realify for F {}

/*
    For a given input type I, if the function outputs a complex number u + vi, then its
    realified form outputs a ⟨u, v⟩ tuple.
        f(z) = u + vi => fᵣ(z) = ⟨u, v⟩
    This allows using derivatives of the complex number components.

    For instance, for f(z) = z² with complex input z = a + bi:
        f(a+bi) = (a+bi)² = (a² - b²) + (2ab)i
        fᵣ(a, b) = ⟨u(a, b), v(a, b)⟩ = ⟨a² - b², 2ab⟩

    We can then take partial derivatives of the components independently:
        ∂u/∂a = 2a
        ∂u/∂b = -2b
        ∂v/∂a = 2b
        ∂v/∂b = 2a
*/
pub(crate) trait Realified<I> {
    type Realified;

    fn realified(&self) -> Self::Realified;
}

impl<F, I> Realified<I> for StaticZero<F>
where
    F: Output<I>,
    F::Output: RealifyStaticConst<Self, F>,
{
    type Realified = <F::Output as RealifyStaticConst<Self, F>>::Realified;

    fn realified(&self) -> Self::Realified {
        <F::Output as RealifyStaticConst<Self, F>>::realify_static_const()
    }
}

impl<F, I> Realified<I> for StaticOne<F>
where
    F: Output<I>,
    F::Output: RealifyStaticConst<Self, F>,
{
    type Realified = <F::Output as RealifyStaticConst<Self, F>>::Realified;

    fn realified(&self) -> Self::Realified {
        <F::Output as RealifyStaticConst<Self, F>>::realify_static_const()
    }
}

impl<F, I> Realified<I> for StaticTwo<F>
where
    F: Output<I>,
    F::Output: RealifyStaticConst<Self, F>,
{
    type Realified = <F::Output as RealifyStaticConst<Self, F>>::Realified;

    fn realified(&self) -> Self::Realified {
        <F::Output as RealifyStaticConst<Self, F>>::realify_static_const()
    }
}

pub(crate) trait RealifyStaticConst<SC, F> {
    type Realified;
    fn realify_static_const() -> Self::Realified;
}
impl<R: Real, SC, F> RealifyStaticConst<SC, F> for R
where
    SC: Default,
{
    type Realified = Real1Function<SC>;

    fn realify_static_const() -> Self::Realified {
        Real1Function(SC::default())
    }
}
impl<R, SC, F> RealifyStaticConst<SC, F> for Complex<R>
where
    SC: Default,
{
    type Realified = ComplexRealFunction<SC, StaticZero<F>>;

    fn realify_static_const() -> Self::Realified {
        ComplexRealFunction {
            real: SC::default(),
            imag: StaticZero::new(),
        }
    }
}

impl<R: Real, I> Realified<I> for TypedConst<R>
where
    Self: Clone,
{
    type Realified = Real1Function<Self>;

    fn realified(&self) -> Self::Realified {
        Real1Function(self.clone())
    }
}
impl<R, I> Realified<I> for TypedConst<Complex<R>>
where
    R: Clone,
{
    type Realified = ComplexRealFunction<TypedConst<R>, TypedConst<R>>;

    fn realified(&self) -> Self::Realified {
        ComplexRealFunction {
            real: TypedConst::new(self.value.real.clone()),
            imag: TypedConst::new(self.value.imag.clone()),
        }
    }
}

impl<N, R: Real> Realified<R> for Variable<0, N>
where
    N: Clone,
{
    type Realified = Real1Function<Self>;

    fn realified(&self) -> Self::Realified {
        Real1Function(self.clone())
    }
}
impl<N, R> Realified<Complex<R>> for Variable<0, N>
where
    N: Display,
{
    type Realified = ComplexRealFunction<Variable<0, String>, Variable<1, String>>;

    fn realified(&self) -> Self::Realified {
        ComplexRealFunction {
            real: Variable::with_name0(format!("{}.real", self.name())),
            imag: Variable::with_name1(format!("{}.imag", self.name())),
        }
    }
}

#[ops_bounds(
    F: Realified<I>,
    -F::Realified,
)]
impl<F, I> Realified<I> for Negation<F> {
    type Realified = ops_type!(-F::Realified);

    fn realified(&self) -> Self::Realified {
        -self.0.realified()
    }
}

#[ops_bounds(
    L: Realified<I>,
    R: Realified<I>,
    L::Realified + R::Realified,
)]
impl<L, R, I> Realified<I> for Addition<L, R> {
    type Realified = ops_type!(L::Realified + R::Realified);

    fn realified(&self) -> Self::Realified {
        self.left.realified() + self.right.realified()
    }
}

#[ops_bounds(
    L: Realified<I>,
    R: Realified<I>,
    L::Realified - R::Realified,
)]
impl<L, R, I> Realified<I> for Subtraction<L, R> {
    type Realified = ops_type!(L::Realified - R::Realified);

    fn realified(&self) -> Self::Realified {
        self.left.realified() - self.right.realified()
    }
}

#[ops_bounds(
    L: Realified<I>,
    R: Realified<I>,
    L::Realified * R::Realified,
)]
impl<L, R, I> Realified<I> for Multiplication<L, R> {
    type Realified = ops_type!(L::Realified * R::Realified);

    fn realified(&self) -> Self::Realified {
        self.left.realified() * self.right.realified()
    }
}

#[ops_bounds(
    L: Realified<I>,
    R: Realified<I>,
    L::Realified / R::Realified,
)]
impl<L, R, I> Realified<I> for Division<L, R> {
    type Realified = ops_type!(L::Realified / R::Realified);

    fn realified(&self) -> Self::Realified {
        self.left.realified() / self.right.realified()
    }
}

#[ops_bounds(
    F: Realified<I>,
    F::Realified ^ 2,
)]
impl<F, I> Realified<I> for Squared<F> {
    type Realified = ops_type!(F::Realified ^ 2);

    fn realified(&self) -> Self::Realified {
        self.0.realified().sqr()
    }
}

impl<F, I> Realified<I> for ModuloSquared<F>
where
    F: Realified<I>,
    F::Realified: ModSqr,
{
    type Realified = <F::Realified as ModSqr>::Output;

    fn realified(&self) -> Self::Realified {
        self.0.realified().mod_sqr()
    }
}
