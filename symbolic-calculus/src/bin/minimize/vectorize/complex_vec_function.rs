use std::fmt::Display;
use std::ops::Sub;

use ops_traits_macros::{ops_bounds, ops_type};
use symbolic_calculus::util::Sqr;
use symbolic_calculus::{Apply, EvaluatedDisplayData, FunctionDisplay, Output, StaticTwo};
use symbolic_calculus_macros::{
    Arity, ComparisonTokens, Display, Function, RenameVariable, SetNamedConstValue, VariableNames,
};

use crate::complex::Complex;
use crate::modulo_squared::ModSqr;
use crate::real::Real;
use crate::vectorize::vec1_function::Vec1Function;

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
#[function(operands = 2)]
pub(crate) struct ComplexVecFunction<R, I> {
    pub real: R,
    pub imag: I,
}

impl<R, I> ComplexVecFunction<R, I> {
    fn new(real: R, imag: I) -> Self {
        ComplexVecFunction { real, imag }
    }
}

impl<R, I, S: Real> Output<S> for ComplexVecFunction<R, I>
where
    R: Output<S>,
    I: Output<S>,
{
    type Output = (R::Output, I::Output);
}
impl<R, I, S: Real> Apply<S> for ComplexVecFunction<R, I>
where
    R: Apply<S>,
    I: Apply<S>,
    S: Clone,
{
    fn apply(this: &Self, input: S) -> Self::Output {
        let real = this.real.apply(input.clone());
        let imag = this.imag.apply(input);
        (real, imag)
    }
}

impl<R, I, R2, I2> Output<(R2, I2)> for ComplexVecFunction<R, I>
where
    R: Output<(R2, I2)>,
    I: Output<(R2, I2)>,
{
    type Output = (R::Output, I::Output);
}
impl<R, I, R2, I2> Apply<(R2, I2)> for ComplexVecFunction<R, I>
where
    R: Apply<(R2, I2)>,
    I: Apply<(R2, I2)>,
    (R2, I2): Clone,
{
    fn apply(this: &Self, input: (R2, I2)) -> Self::Output {
        let real = this.real.apply(input.clone());
        let imag = this.imag.apply(input);
        (real, imag)
    }
}

impl<R, I, S, O> Output<Complex<S>> for ComplexVecFunction<R, I>
where
    R: Output<(S, S), Output = O>,
    I: Output<(S, S), Output = O>,
{
    type Output = Complex<O>;
}
impl<R, I, S, O> Apply<Complex<S>> for ComplexVecFunction<R, I>
where
    R: Apply<(S, S), Output = O>,
    I: Apply<(S, S), Output = O>,
    S: Clone,
{
    fn apply(this: &Self, input: Complex<S>) -> Self::Output {
        Complex {
            real: this.real.apply((input.real.clone(), input.imag.clone())),
            imag: this.imag.apply((input.real, input.imag)),
        }
    }
}

impl<R, I> FunctionDisplay for ComplexVecFunction<R, I> {
    // TODO: value.
    const PRECEDENCE: u8 = 0;

    fn evaluate_display_data(&self) -> EvaluatedDisplayData<impl Display> {
        Self::non_const_evaluated_display_data(todo!())
    }
}

// TODO: clean-up
// impl<R: Display, I: Display> Display for ComplexVecFunction<R, I> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "⟨{}, {}⟩", self.real, self.imag)
//     }
// }

// ComplexVecFunction<R, I> - ComplexVecFunction<R2, I2>
#[ops_bounds(
    R - R2,
    I - I2,
)]
impl<R, I, R2, I2> Sub<ComplexVecFunction<R2, I2>> for ComplexVecFunction<R, I> {
    type Output = ComplexVecFunction<ops_type!(R - R2), ops_type!(I - I2)>;

    fn sub(self, rhs: ComplexVecFunction<R2, I2>) -> Self::Output {
        ComplexVecFunction {
            real: self.real - rhs.real,
            imag: self.imag - rhs.imag,
        }
    }
}

// ComplexVecFunction<R, I>²
#[ops_bounds(
    R: Clone,
    I: Clone,
    // 2 * (a * b)
    StaticTwo<ops_type!(R * I)> * (R * I),
    // (a²) - (b²)
    R ^ 2 - I ^ 2,
)]
impl<R, I> Sqr for ComplexVecFunction<R, I> {
    type Output = ComplexVecFunction<
        ops_type!(R ^ 2 - I ^ 2),
        ops_type!(StaticTwo<ops_type!(R * I)> * (R * I)),
    >;

    fn sqr(self) -> Self::Output {
        let ComplexVecFunction { real: a, imag: b } = self;
        /*
           (a+bi)² = a² + 2abi + (bi)²
                   = a² - b² + 2abi
        */
        ComplexVecFunction {
            real: a.clone().sqr() - b.clone().sqr(),
            imag: StaticTwo::new() * (a * b),
        }
    }
}

// |ComplexVecFunction<R, I>|²
#[ops_bounds(R ^ 2 + I ^ 2)]
impl<R, I> ModSqr for ComplexVecFunction<R, I> {
    type Output = Vec1Function<ops_type!(R ^ 2 + I ^ 2)>;

    fn mod_sqr(self) -> Self::Output {
        Vec1Function(self.real.sqr() + self.imag.sqr())
    }
}

// TODO: other ops
