mod complex_vec_function;
mod vec1_function;

use std::fmt::Display;

use ops_traits_macros::{ops_bounds, ops_type};
use symbolic_calculus::util::Sqr;
use symbolic_calculus::{Division, Function, Squared, Subtraction, TypedConst, Variable};
pub(crate) use vec1_function::Gradient;

use crate::complex::Complex;
use crate::modulo_squared::{ModSqr, ModuloSquared};
use crate::real::Real;
use crate::vectorize::complex_vec_function::ComplexVecFunction;
use crate::vectorize::vec1_function::Vec1Function;

pub(crate) trait Vectorize: Function {
    fn vectorize<I>(&self) -> <Self as Vectorized<I>>::Vectorized
    where
        Self: Vectorized<I>,
    {
        self.vectorized()
    }
}
impl<F: Function> Vectorize for F {}

/*
   For a given input type I, if the function outputs a complex number a+bi, then its
   vectorized form outputs a ⟨a, b⟩ tuple.
       f(x) = a+bi => fvec(x) = ⟨a, b⟩
   This allows using derivatives of the complex number components.

   For instance, for f(x) = x², its derivative is f'(x) = 2x. For x = a+bi, then
   f'(a+bi) = 2a + 2bi

   But for complex numbers, f(a+bi) = (a+bi)² = a² - b² + 2abi. Then, fvec(a+bi) = ⟨a² - b², 2ab⟩, and
   fvec'(a+bi) = ⟨2a, 2a⟩, which corresponds to 2a + 2ai.
*/
pub(crate) trait Vectorized<I> {
    type Vectorized;

    fn vectorized(&self) -> Self::Vectorized;
}

impl<R: Real, I> Vectorized<I> for TypedConst<R>
where
    Self: Clone,
{
    type Vectorized = Vec1Function<Self>;

    fn vectorized(&self) -> Self::Vectorized {
        Vec1Function(self.clone())
    }
}
impl<R, I> Vectorized<I> for TypedConst<Complex<R>>
where
    R: Clone,
{
    type Vectorized = ComplexVecFunction<TypedConst<R>, TypedConst<R>>;

    fn vectorized(&self) -> Self::Vectorized {
        ComplexVecFunction {
            real: TypedConst::new(self.value.real.clone()),
            imag: TypedConst::new(self.value.imag.clone()),
        }
    }
}

impl<N, R: Real> Vectorized<R> for Variable<0, N>
where
    N: Clone,
{
    type Vectorized = Vec1Function<Self>;

    fn vectorized(&self) -> Self::Vectorized {
        Vec1Function(self.clone())
    }
}
impl<N, R> Vectorized<Complex<R>> for Variable<0, N>
where
    N: Display,
{
    type Vectorized = ComplexVecFunction<Variable<0, String>, Variable<1, String>>;

    fn vectorized(&self) -> Self::Vectorized {
        ComplexVecFunction {
            real: Variable::with_name0(format!("{}.real", self.name())),
            imag: Variable::with_name1(format!("{}.imag", self.name())),
        }
    }
}

#[ops_bounds(
    L: Vectorized<I>,
    R: Vectorized<I>,
    L::Vectorized - R::Vectorized,
)]
impl<L, R, I> Vectorized<I> for Subtraction<L, R> {
    type Vectorized = ops_type!(L::Vectorized - R::Vectorized);

    fn vectorized(&self) -> Self::Vectorized {
        self.left.vectorized() - self.right.vectorized()
    }
}

#[ops_bounds(
    L: Vectorized<I>,
    R: Vectorized<I>,
    L::Vectorized / R::Vectorized,
)]
impl<L, R, I> Vectorized<I> for Division<L, R> {
    type Vectorized = ops_type!(L::Vectorized / R::Vectorized);

    fn vectorized(&self) -> Self::Vectorized {
        self.left.vectorized() / self.right.vectorized()
    }
}

#[ops_bounds(
    F: Vectorized<I>,
    F::Vectorized ^ 2,
)]
impl<F, I> Vectorized<I> for Squared<F> {
    type Vectorized = ops_type!(F::Vectorized ^ 2);

    fn vectorized(&self) -> Self::Vectorized {
        self.0.vectorized().sqr()
    }
}

impl<F, I> Vectorized<I> for ModuloSquared<F>
where
    F: Vectorized<I>,
    F::Vectorized: ModSqr,
{
    type Vectorized = <F::Vectorized as ModSqr>::Output;

    fn vectorized(&self) -> Self::Vectorized {
        self.0.vectorized().mod_sqr()
    }
}

// TODO: other impls
