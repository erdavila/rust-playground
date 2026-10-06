use core::fmt;
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Neg, Sub};

use ops_traits_macros::{ops_bounds, ops_type};
use symbolic_calculus::util::Sqr;
use symbolic_calculus::{AlwaysConst, Apply, EvaluatedDisplayData, FunctionDisplay, Output};
use symbolic_calculus_macros::{
    Arity, ComparisonTokens, Display, Function, RenameVariable, SetNamedConstValue, VariableNames,
};

use crate::complex::{Complex, RealWrapper};
use crate::modulo_squared::ModSqr;
use crate::real::Real;
use crate::realification::real_1_function::Real1Function;

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
pub(crate) struct ComplexRealFunction<R, I> {
    pub real: R,
    pub imag: I,
}

impl<R, I> ComplexRealFunction<R, I> {
    fn new(real: R, imag: I) -> Self {
        ComplexRealFunction { real, imag }
    }
}

impl<R, I, S: Real> Output<S> for ComplexRealFunction<R, I>
where
    R: Output<S>,
    I: Output<S>,
{
    type Output = (R::Output, I::Output);
}
impl<R, I, S: Real> Apply<S> for ComplexRealFunction<R, I>
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

impl<R, I, R2, I2> Output<(R2, I2)> for ComplexRealFunction<R, I>
where
    R: Output<(R2, I2)>,
    I: Output<(R2, I2)>,
{
    type Output = (R::Output, I::Output);
}
impl<R, I, R2, I2> Apply<(R2, I2)> for ComplexRealFunction<R, I>
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

impl<R, I, S, O> Output<Complex<S>> for ComplexRealFunction<R, I>
where
    R: Output<(S, S), Output = O>,
    I: Output<(S, S), Output = O>,
{
    type Output = Complex<O>;
}
impl<R, I, S, O> Apply<Complex<S>> for ComplexRealFunction<R, I>
where
    R: Apply<(S, S), Output = O>,
    I: Apply<(S, S), Output = O>,
    S: Clone,
{
    fn apply(this: &Self, input: Complex<S>) -> Self::Output {
        Complex {
            real: this.real.apply(input.clone().into_tuple()),
            imag: this.imag.apply(input.into_tuple()),
        }
    }
}

impl<R, I> FunctionDisplay for ComplexRealFunction<R, I>
where
    R: FunctionDisplay,
    I: FunctionDisplay,
{
    const PRECEDENCE: u8 = u8::MAX;

    fn evaluated_display_data(&self) -> EvaluatedDisplayData<impl Display> {
        let real = self.real.evaluated_display_data();
        let imag = self.imag.evaluated_display_data();

        let always_const = if imag.always_const == Some(AlwaysConst::Zero) {
            real.always_const
        } else {
            None
        };

        let real_display = real.display_data.display;
        let imag_display = imag.display_data.display;

        Self::maybe_const_evaluated_display_data(
            always_const,
            fmt::from_fn(move |f| write!(f, "⟨{real_display}, {imag_display}⟩")),
        )
    }
}

pub(super) trait IntoComplex {
    type Complex;
    fn into_complex(self) -> Self::Complex;
}
impl<R, I> IntoComplex for ComplexRealFunction<R, I> {
    type Complex = Complex<R, I>;

    fn into_complex(self) -> Self::Complex {
        Complex {
            real: self.real,
            imag: self.imag,
        }
    }
}

pub(crate) trait IntoComplexRealFunction {
    type ComplexRealFunction;
    fn into_complex_real_function(self) -> Self::ComplexRealFunction;
}
impl<R, I> IntoComplexRealFunction for Complex<R, I> {
    type ComplexRealFunction = ComplexRealFunction<R, I>;

    fn into_complex_real_function(self) -> Self::ComplexRealFunction {
        ComplexRealFunction {
            real: self.real,
            imag: self.imag,
        }
    }
}

pub(super) type FromComplex<T> = <T as IntoComplexRealFunction>::ComplexRealFunction;

// -ComplexRealFunction<R, I>
#[ops_bounds(-Complex<R, I>: IntoComplexRealFunction)]
impl<R, I> Neg for ComplexRealFunction<R, I> {
    type Output = FromComplex<ops_type!(-Complex<R, I>)>;

    fn neg(self) -> Self::Output {
        self.as_complex(Neg::neg)
    }
}

// ComplexRealFunction<R, I> + ComplexRealFunction<R2, I2>
#[ops_bounds(Complex<R, I> + Complex<R2, I2>: IntoComplexRealFunction)]
impl<R, I, R2, I2> Add<ComplexRealFunction<R2, I2>> for ComplexRealFunction<R, I> {
    type Output = FromComplex<ops_type!(Complex<R, I> + Complex<R2, I2>)>;

    fn add(self, rhs: ComplexRealFunction<R2, I2>) -> Self::Output {
        (self, rhs).as_complexes(Add::add)
    }
}

// ComplexRealFunction<R, I> + Real1Function<S>
#[ops_bounds(Complex<R, I> + RealWrapper<S>: IntoComplexRealFunction)]
impl<R, I, S> Add<Real1Function<S>> for ComplexRealFunction<R, I> {
    type Output = FromComplex<ops_type!(Complex<R, I> + RealWrapper<S>)>;

    fn add(self, rhs: Real1Function<S>) -> Self::Output {
        let r = RealWrapper(rhs.0);
        self.as_complex(|l| l + r)
    }
}

// ComplexRealFunction<R, I> - ComplexRealFunction<R2, I2>
#[ops_bounds(Complex<R, I> - Complex<R2, I2>: IntoComplexRealFunction)]
impl<R, I, R2, I2> Sub<ComplexRealFunction<R2, I2>> for ComplexRealFunction<R, I> {
    type Output = FromComplex<ops_type!(Complex<R, I> - Complex<R2, I2>)>;

    fn sub(self, rhs: ComplexRealFunction<R2, I2>) -> Self::Output {
        (self, rhs).as_complexes(Sub::sub)
    }
}

// ComplexRealFunction<R, I> - Real1Function<S>
#[ops_bounds(Complex<R, I> - RealWrapper<S>: IntoComplexRealFunction)]
impl<R, I, S> Sub<Real1Function<S>> for ComplexRealFunction<R, I> {
    type Output = FromComplex<ops_type!(Complex<R, I> - RealWrapper<S>)>;

    fn sub(self, rhs: Real1Function<S>) -> Self::Output {
        let r = RealWrapper(rhs.0);
        self.as_complex(|l| l - r)
    }
}

// ComplexRealFunction<R, I> * ComplexRealFunction<R2, I2>
#[ops_bounds(Complex<R, I> * Complex<R2, I2>: IntoComplexRealFunction)]
impl<R, I, R2, I2> Mul<ComplexRealFunction<R2, I2>> for ComplexRealFunction<R, I> {
    type Output = FromComplex<ops_type!(Complex<R, I> * Complex<R2, I2>)>;

    fn mul(self, rhs: ComplexRealFunction<R2, I2>) -> Self::Output {
        (self, rhs).as_complexes(Mul::mul)
    }
}

// ComplexRealFunction<R, I> * Real1Function<S>
#[ops_bounds(Complex<R, I> * RealWrapper<S>: IntoComplexRealFunction)]
impl<R, I, S> Mul<Real1Function<S>> for ComplexRealFunction<R, I> {
    type Output = FromComplex<ops_type!(Complex<R, I> * RealWrapper<S>)>;

    fn mul(self, rhs: Real1Function<S>) -> Self::Output {
        let r = RealWrapper(rhs.0);
        self.as_complex(|l| l * r)
    }
}

// ComplexRealFunction<R, I> / ComplexRealFunction<R2, I2>
#[ops_bounds(Complex<R, I> / Complex<R2, I2>: IntoComplexRealFunction)]
impl<R, I, R2, I2> Div<ComplexRealFunction<R2, I2>> for ComplexRealFunction<R, I> {
    type Output = FromComplex<ops_type!(Complex<R, I> / Complex<R2, I2>)>;

    fn div(self, rhs: ComplexRealFunction<R2, I2>) -> Self::Output {
        (self, rhs).as_complexes(Div::div)
    }
}

// ComplexRealFunction<R, I> / Real1Function<S>
#[ops_bounds(Complex<R, I> / RealWrapper<S>: IntoComplexRealFunction)]
impl<R, I, S> Div<Real1Function<S>> for ComplexRealFunction<R, I> {
    type Output = FromComplex<ops_type!(Complex<R, I> / RealWrapper<S>)>;

    fn div(self, rhs: Real1Function<S>) -> Self::Output {
        let r = RealWrapper(rhs.0);
        self.as_complex(|l| l / r)
    }
}

// ComplexRealFunction<R, I>²
#[ops_bounds(Complex<R, I> ^ 2: IntoComplexRealFunction)]
impl<R, I> Sqr for ComplexRealFunction<R, I> {
    type Output = FromComplex<ops_type!(Complex<R, I> ^ 2)>;

    fn sqr(self) -> Self::Output {
        self.as_complex(Sqr::sqr)
    }
}

// |ComplexRealFunction<R, I>|²
impl<R, I> ModSqr for ComplexRealFunction<R, I>
where
    Complex<R, I>: ModSqr,
{
    type Output = Real1Function<<Complex<R, I> as ModSqr>::Output>;

    fn mod_sqr(self) -> Self::Output {
        let z = self.into_complex();
        Real1Function(z.mod_sqr())
    }
}

pub(super) trait AsComplex {
    type Complex;

    #[expect(clippy::wrong_self_convention)]
    fn as_complex<T>(self, f: impl FnOnce(Self::Complex) -> T) -> T::ComplexRealFunction
    where
        T: IntoComplexRealFunction;
}
impl<R, I> AsComplex for ComplexRealFunction<R, I> {
    type Complex = Complex<R, I>;

    fn as_complex<T>(self, f: impl FnOnce(Self::Complex) -> T) -> T::ComplexRealFunction
    where
        T: IntoComplexRealFunction,
    {
        let z = self.into_complex();
        f(z).into_complex_real_function()
    }
}

pub(super) trait AsComplexes {
    type Complex1;
    type Complex2;

    #[expect(clippy::wrong_self_convention)]
    fn as_complexes<T>(
        self,
        f: impl FnOnce(Self::Complex1, Self::Complex2) -> T,
    ) -> T::ComplexRealFunction
    where
        T: IntoComplexRealFunction;
}
impl<R1, I1, R2, I2> AsComplexes for (ComplexRealFunction<R1, I1>, ComplexRealFunction<R2, I2>) {
    type Complex1 = Complex<R1, I1>;
    type Complex2 = Complex<R2, I2>;

    fn as_complexes<T>(
        self,
        f: impl FnOnce(Self::Complex1, Self::Complex2) -> T,
    ) -> T::ComplexRealFunction
    where
        T: IntoComplexRealFunction,
    {
        let z1 = self.0.into_complex();
        let z2 = self.1.into_complex();
        f(z1, z2).into_complex_real_function()
    }
}

#[cfg(test)]
mod tests {
    use symbolic_calculus::{Function, Variable};

    use super::*;
    use crate::realification::Realify as _;

    #[test]
    fn sqr() {
        let function1 = Variable::new0().realify::<Complex<f64>>().sqr();
        let function2 = Variable::new0().sqr().realify::<Complex<f64>>();

        let expected_string = "⟨x.real² - x.imag², 2 * (x.real * x.imag)⟩";
        assert_eq!(function1.to_string(), expected_string);
        assert_eq!(function2.to_string(), expected_string);

        let complex_input = Complex::new(4.0, 3.0);
        let expected_complex_output = complex_input.sqr();
        assert_eq!(function1.apply(complex_input), expected_complex_output);
        assert_eq!(function2.apply(complex_input), expected_complex_output);

        let tuple_input = complex_input.into_tuple();
        let expected_tuple_output = expected_complex_output.into_tuple();
        assert_eq!(function1.apply(tuple_input), expected_tuple_output);
        assert_eq!(function2.apply(tuple_input), expected_tuple_output);
    }
}
