use std::fmt::{self, Display};
use std::ops::{Add, Div, Mul, Neg, Sub};

use math::util::Sqr;
use math::{
    Apply, Derivative, Division, Function, Negation, Output, RenameVariable, Squared, Subtraction,
    VariableNames,
};

use crate::complex::Complex;
use crate::modulo_squared::ModSqr;
use crate::real::Real;
use crate::vectorize::complex_vec_function::ComplexVecFunction;

pub(crate) struct Vec1Function<F>(pub F);

impl<F> Function for Vec1Function<F> {}

impl<F, R: Real> Output<R> for Vec1Function<F>
where
    F: Output<R>,
{
    type Output = F::Output;
}
impl<F, R: Real> Apply<R> for Vec1Function<F>
where
    F: Apply<R>,
{
    fn apply(this: &Self, input: R) -> Self::Output {
        this.0.apply(input)
    }
}

impl<F, R> Output<Complex<R>> for Vec1Function<F>
where
    F: Output<(R, R)>,
{
    type Output = F::Output;
}
impl<F, R> Apply<Complex<R>> for Vec1Function<F>
where
    F: Apply<(R, R)>,
{
    fn apply(this: &Self, input: Complex<R>) -> Self::Output {
        this.0.apply((input.real, input.imag))
    }
}

impl<F, R> Output<(R,)> for Vec1Function<F>
where
    F: Output<R>,
{
    type Output = (F::Output,);
}
impl<F, R> Apply<(R,)> for Vec1Function<F>
where
    F: Apply<R>,
{
    fn apply(this: &Self, (input,): (R,)) -> Self::Output {
        (this.0.apply(input),)
    }
}

impl<F, R> Output<(R, R)> for Vec1Function<F>
where
    F: Output<(R, R)>,
{
    type Output = F::Output;
}
impl<F, R> Apply<(R, R)> for Vec1Function<F>
where
    F: Apply<(R, R)>,
{
    fn apply(this: &Self, input: (R, R)) -> Self::Output {
        this.0.apply(input)
    }
}

impl<F, const V: usize> Derivative<V> for Vec1Function<F>
where
    F: Derivative<V>,
{
    type Derivative = Vec1Function<F::Derivative>;

    fn derivative(this: &Self) -> Self::Derivative {
        Vec1Function(this.0.derivative())
    }
}

impl<F, const V: usize> RenameVariable<V> for Vec1Function<F>
where
    F: RenameVariable<V>,
{
    type RenamedVar<N> = Vec1Function<F::RenamedVar<N>>;

    fn rename_var_num<N: Clone>(this: &Self, name: N) -> Self::RenamedVar<N> {
        Vec1Function(this.0.rename_var_num(name))
    }
}

impl<F> VariableNames for Vec1Function<F>
where
    F: VariableNames,
{
    fn var_names(&self) -> impl Iterator<Item = (usize, &dyn Display)> {
        self.0.var_names()
    }
}

impl<F: Display> Display for Vec1Function<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "⟨{}⟩", self.0)
    }
}

pub(crate) trait Gradient<I>: Function {
    type Gradient;

    fn gradient(&self) -> Self::Gradient;
}
impl<F, R: Real> Gradient<R> for Vec1Function<F>
where
    F: Derivative<0>,
{
    type Gradient = Vec1Function<F::Derivative>;

    fn gradient(&self) -> Self::Gradient {
        Vec1Function(self.0.derivative())
    }
}
impl<F, R> Gradient<Complex<R>> for Vec1Function<F>
where
    F: Derivative<0> + Derivative<1>,
{
    type Gradient =
        ComplexVecFunction<<F as Derivative<0>>::Derivative, <F as Derivative<1>>::Derivative>;

    fn gradient(&self) -> Self::Gradient {
        ComplexVecFunction {
            real: self.0.derivative::<0>(),
            imag: self.0.derivative::<1>(),
        }
    }
}

// Vec1Function<F> - Vec1Function<G>
impl<F, F2> Sub<Vec1Function<F2>> for Vec1Function<F> {
    type Output = Vec1Function<Subtraction<F, F2>>;

    fn sub(self, rhs: Vec1Function<F2>) -> Self::Output {
        Vec1Function(Subtraction {
            left: self.0,
            right: rhs.0,
        })
    }
}

// Vec1Function<F> - ComplexVecFunction<R, I>
impl<F, R, I> Sub<ComplexVecFunction<R, I>> for Vec1Function<F> {
    type Output = ComplexVecFunction<Subtraction<F, R>, Negation<I>>;

    fn sub(self, rhs: ComplexVecFunction<R, I>) -> Self::Output {
        /*
           x - (a+bi) = (x - a) + (-b)i
        */
        ComplexVecFunction {
            // x - a
            real: Subtraction {
                // x
                left: self.0,
                // a
                right: rhs.real,
            },
            // -b
            imag: Negation(rhs.imag),
        }
    }
}

// Vec1Function<F> / Vec1Function<G>
impl<F, G> Div<Vec1Function<G>> for Vec1Function<F> {
    type Output = Vec1Function<Division<F, G>>;

    fn div(self, rhs: Vec1Function<G>) -> Self::Output {
        Vec1Function(Division {
            left: self.0,
            right: rhs.0,
        })
    }
}

// Vec1Function<F> / ComplexVecFunction<R, I>
impl<F, R, I> Div<ComplexVecFunction<R, I>> for Vec1Function<F>
where
    F: Clone,
    // a²
    R: Sqr + Clone,
    // b²
    I: Sqr + Clone,
    // (a²) + (b²)
    <R as Sqr>::Output: Add<<I as Sqr>::Output>,
    // x * a
    F: Mul<R>,
    // -x
    F: Neg,
    // (x * a) / (a² + b²)
    <F as Mul<R>>::Output: Div<<<R as Sqr>::Output as Add<<I as Sqr>::Output>>::Output>,
    // (-x) * b
    <F as Neg>::Output: Mul<I>,
    // (-x * b) / (a² + b²)
    <<F as Neg>::Output as Mul<I>>::Output:
        Div<<<R as Sqr>::Output as Add<<I as Sqr>::Output>>::Output>,
    // (a² + b²).clone()
    <<R as Sqr>::Output as Add<<I as Sqr>::Output>>::Output: Clone,
{
    type Output =
        ComplexVecFunction<
            <<F as Mul<R>>::Output as Div<
                <<R as Sqr>::Output as Add<<I as Sqr>::Output>>::Output,
            >>::Output,
            <<<F as Neg>::Output as Mul<I>>::Output as Div<
                <<R as Sqr>::Output as Add<<I as Sqr>::Output>>::Output,
            >>::Output,
        >;

    fn div(self, rhs: ComplexVecFunction<R, I>) -> Self::Output {
        let Vec1Function(x) = self;
        let ComplexVecFunction { real: a, imag: b } = rhs;
        /*
           x / (a+bi) = x.(a-bi) / [(a+bi).(a-bi)]
                     = (xa - xbi) / (a² + b²)
                     = xa / (a² + b²) - [xb / (a² + b²)]i
        */

        let divisor = a.clone().sqr() + b.clone().sqr();

        ComplexVecFunction {
            real: x.clone() * a / divisor.clone(),
            imag: -x * b / divisor,
        }
    }
}

// |Vec1Function<F>|²
impl<F> ModSqr for Vec1Function<F> {
    type Output = Vec1Function<Squared<F>>;

    fn mod_sqr(self) -> Self::Output {
        Vec1Function(Squared(self.0))
    }
}
