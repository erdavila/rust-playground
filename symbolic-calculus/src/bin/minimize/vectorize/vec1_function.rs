use std::fmt::Display;
use std::ops::{Div, Sub};

use ops_traits_macros::{ops_bounds, ops_type};
use symbolic_calculus::util::Sqr;
use symbolic_calculus::{
    Apply, Derivative, Division, EvaluatedDisplayData, Function, FunctionDisplay, Negation, Output,
    Squared, Subtraction,
};
use symbolic_calculus_macros::{
    Arity, ComparisonTokens, Display, Function, RenameVariable, SetNamedConstValue, VariableNames,
};

use crate::complex::Complex;
use crate::modulo_squared::ModSqr;
use crate::real::Real;
use crate::vectorize::complex_vec_function::ComplexVecFunction;

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
pub(crate) struct Vec1Function<F>(pub F);

impl<F> Vec1Function<F> {
    fn new(func: F) -> Self {
        Vec1Function(func)
    }
}

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

impl<F> FunctionDisplay for Vec1Function<F> {
    // TODO: value.
    const PRECEDENCE: u8 = 0;

    fn evaluate_display_data(&self) -> EvaluatedDisplayData<impl Display> {
        Self::non_const_evaluated_display_data(todo!())
    }
}

// impl<F: Display> Display for Vec1Function<F> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "⟨{}⟩", self.0)
//     }
// }

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

// Vec1Function<F> - Vec1Function<F2>
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
#[ops_bounds(
    F: Clone,
    R: Clone,
    I: Clone,
    // (a²) + (b²)
    R ^ 2 + I ^ 2 = D,
    // (a² + b²).clone()
    D: Clone,
    // (x * a) / (a² + b²)
    F * R / D,
    // (-x * b) / (a² + b²)
    -F * I / D,
)]
impl<F, R, I, D> Div<ComplexVecFunction<R, I>> for Vec1Function<F> {
    type Output = ComplexVecFunction<ops_type!(F * R / D), ops_type!(-F * I / D)>;

    fn div(self, rhs: ComplexVecFunction<R, I>) -> Self::Output {
        let Vec1Function(x) = self;
        let ComplexVecFunction { real: a, imag: b } = rhs;
        /*
             x          x.(a-bi)   xa - xbi     xa        xb
            ---- = ------------- = -------- = ------- - ------- i
            a+bi   (a+bi).(a-bi)   a² + b²    a² + b²   a² + b²
        */

        let denominator = a.clone().sqr() + b.clone().sqr();

        ComplexVecFunction {
            real: x.clone() * a / denominator.clone(),
            imag: -x * b / denominator,
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

// TODO: other ops
