use std::fmt::{self, Display};
use std::ops::{Add, Mul, Sub};

use math::util::{MergeVarNames, Sqr};
use math::{Apply, Function, Output, RenameVariable, StaticTwo, VariableNames};

use crate::complex::Complex;
use crate::modulo_squared::ModSqr;
use crate::real::Real;
use crate::vectorize::vec1_function::Vec1Function;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ComplexVecFunction<R, I> {
    pub real: R,
    pub imag: I,
}

impl<R, I> Function for ComplexVecFunction<R, I> {}

impl<R, I, S: Real> Output<S> for ComplexVecFunction<R, I> {
    type Output = Self;
}
impl<R, I, S: Real> Apply<S> for ComplexVecFunction<R, I> {
    fn apply(this: &Self, input: S) -> Self::Output {
        todo!()
    }
}

impl<R, I, R2, I2> Output<(R2, I2)> for ComplexVecFunction<R, I>
where
    R: Output<(R2, I2)>,
    I: Output<(R2, I2)>,
{
    type Output = (
        <R as Output<(R2, I2)>>::Output,
        <I as Output<(R2, I2)>>::Output,
    );
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

impl<R, I, const V: usize> RenameVariable<V> for ComplexVecFunction<R, I>
where
    R: RenameVariable<V>,
    I: RenameVariable<V>,
{
    type RenamedVar<N> = ComplexVecFunction<R::RenamedVar<N>, I::RenamedVar<N>>;

    fn rename_var_num<N: Clone>(this: &Self, name: N) -> Self::RenamedVar<N> {
        ComplexVecFunction {
            real: this.real.rename_var_num(name.clone()),
            imag: this.imag.rename_var_num(name),
        }
    }
}

impl<R, I> VariableNames for ComplexVecFunction<R, I>
where
    R: VariableNames,
    I: VariableNames,
{
    fn var_names(&self) -> impl Iterator<Item = (usize, &dyn Display)> {
        MergeVarNames::new(self.real.var_names(), self.imag.var_names())
    }
}

impl<R: Display, I: Display> Display for ComplexVecFunction<R, I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "⟨{}, {}⟩", self.real, self.imag)
    }
}

// ComplexVecFunction<R, I> - ComplexVecFunction<R2, I2>
impl<R, I, R2, I2> Sub<ComplexVecFunction<R2, I2>> for ComplexVecFunction<R, I>
where
    R: Sub<R2>,
    I: Sub<I2>,
{
    type Output = ComplexVecFunction<<R as Sub<R2>>::Output, <I as Sub<I2>>::Output>;

    fn sub(self, rhs: ComplexVecFunction<R2, I2>) -> Self::Output {
        ComplexVecFunction {
            real: self.real - rhs.real,
            imag: self.imag - rhs.imag,
        }
    }
}

// ComplexVecFunction<R, I>²
impl<R, I> Sqr for ComplexVecFunction<R, I>
where
    // a²
    R: Sqr + Clone,
    // b²
    I: Sqr + Clone,
    // (a²) - (b²)
    <R as Sqr>::Output: Sub<<I as Sqr>::Output>,
    // a * b
    R: Mul<I>,
    // 2 * (a * b)
    StaticTwo<<R as Mul<I>>::Output>: Mul<<R as Mul<I>>::Output>,
    // <R as Sqr>::Output: Sub<<I as Sqr>::Output>,
    // R: Mul<I>,
{
    type Output = ComplexVecFunction<
        <<R as Sqr>::Output as Sub<<I as Sqr>::Output>>::Output,
        <StaticTwo<<R as Mul<I>>::Output> as Mul<<R as Mul<I>>::Output>>::Output,
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
impl<R, I> ModSqr for ComplexVecFunction<R, I>
where
    R: Sqr,
    I: Sqr,
    <R as Sqr>::Output: Add<<I as Sqr>::Output>,
{
    type Output = Vec1Function<<<R as Sqr>::Output as Add<<I as Sqr>::Output>>::Output>;

    fn mod_sqr(self) -> Self::Output {
        Vec1Function(self.real.sqr() + self.imag.sqr())
    }
}
