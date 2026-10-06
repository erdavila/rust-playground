use std::fmt::Display;

use ops_traits_macros::{ops_bounds, ops_type};
use symbolic_calculus::util::{OperationAssociativity, Sqr, UnaryOperation};
use symbolic_calculus::{
    AlwaysConst, Apply, EvaluatedDisplayData, FunctionDisplay, Output, display_enum, impl_ops_for,
};
use symbolic_calculus_macros::{
    Arity, ComparisonTokens, Display, Function, RenameVariable, SetNamedConstValue, VariableNames,
};

use crate::complex::Complex;
use crate::real::Real;

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
pub(crate) struct ModuloSquared<F>(pub F);

impl<F> ModuloSquared<F> {
    pub const fn new(func: F) -> Self {
        ModuloSquared(func)
    }
}

impl<F, I, O> Output<I> for ModuloSquared<F>
where
    F: Output<I>,
    F::Output: ModSqr<Output = O>,
{
    type Output = O;
}

impl<F, I> Apply<I> for ModuloSquared<F>
where
    F: Apply<I>,
    F::Output: ModSqr,
{
    fn apply(this: &Self, input: I) -> Self::Output {
        this.0.apply(input).mod_sqr()
    }
}

/*
   The derivative of f(x) = |x|² cannot be defined for any general input.
   If needed, it must first be vectorized for some given input.
*/
// impl<F, VarId> Derivative<VarId> for ModuloSquared<F> { ... }

impl<F> FunctionDisplay for ModuloSquared<F>
where
    F: FunctionDisplay,
{
    const PRECEDENCE: u8 = 0;

    fn evaluate_display_data(&self) -> EvaluatedDisplayData<impl Display> {
        display_enum! {
            Case {
                Operand,
                NonSimplified,
            }
        }

        let func = self.0.evaluate_display_data();
        match func.always_const {
            // |0|² = 0, |1|² = 1
            Some(AlwaysConst::Zero | AlwaysConst::One) => func.map(Case::Operand),
            // |x|²
            None => Self::unop_evaluated_display_data(func.display_data).map(Case::NonSimplified),
        }
    }
}

impl<F> UnaryOperation<F> for ModuloSquared<F> {
    const DISPLAY_PREFIX: &str = "|";
    const DISPLAY_SUFFIX: &str = "|²";
    const ASSOCIATIVITY: OperationAssociativity = OperationAssociativity::LeftToRight;
}

impl_ops_for!(ModuloSquared<F>);

pub(crate) trait ModSqr {
    type Output;
    fn mod_sqr(self) -> Self::Output;
}
#[ops_bounds(
    R ^ 2 + _,
    R: Clone,
)]
impl<R: Real> ModSqr for R {
    type Output = ops_type!(R ^ 2);

    fn mod_sqr(self) -> Self::Output {
        self.sqr()
    }
}
#[ops_bounds(R ^ 2 + _)]
impl<R> ModSqr for Complex<R> {
    type Output = ops_type!(R ^ 2 + _);

    fn mod_sqr(self) -> Self::Output {
        self.real.sqr() + self.imag.sqr()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod function_display {
        use symbolic_calculus::util::FixedType;
        use symbolic_calculus::{
            Addition, Division, Multiplication, Negation, Squared, StaticOne, StaticZero,
            Subtraction, TypedConst, Variable,
        };

        use super::*;

        type OtherConstType = TypedConst<f64>;
        type VariableType = Variable<0, &'static str>;

        const ZERO: StaticZero<FixedType<f64>> = StaticZero::new();
        const ONE: StaticOne<FixedType<f64>> = StaticOne::new();
        const OTHER: OtherConstType = TypedConst::new(7.0);

        const VAR: VariableType = Variable::with_name0("x");
        const NEG: Negation<OtherConstType> = Negation::new(OTHER);
        const ADD: Addition<OtherConstType, VariableType> = Addition::new(OTHER, VAR);
        const SUB: Subtraction<VariableType, OtherConstType> = Subtraction::new(VAR, OTHER);
        const MUL: Multiplication<OtherConstType, VariableType> = Multiplication::new(OTHER, VAR);
        const DIV: Division<VariableType, OtherConstType> = Division::new(VAR, OTHER);
        const SQR: Squared<OtherConstType> = Squared::new(OTHER);
        const MOD_SQR: ModuloSquared<OtherConstType> = ModuloSquared::new(OTHER);

        #[test]
        fn simplification() {
            assert_eq!(ModuloSquared::new(ZERO).to_string(), "0");
            assert_eq!(ModuloSquared::new(ONE).to_string(), "1");
            assert_eq!(ModuloSquared::new(VAR).to_string(), "|x|²");
        }

        #[test]
        fn precedence() {
            assert_eq!(ModuloSquared::new(VAR).to_string(), "|x|²");
            assert_eq!(ModuloSquared::new(NEG).to_string(), "|-7|²");
            assert_eq!(ModuloSquared::new(ADD).to_string(), "|7 + x|²");
            assert_eq!(ModuloSquared::new(SUB).to_string(), "|x - 7|²");
            assert_eq!(ModuloSquared::new(MUL).to_string(), "|7 * x|²");
            assert_eq!(ModuloSquared::new(DIV).to_string(), "|x / 7|²");
            assert_eq!(ModuloSquared::new(SQR).to_string(), "|7²|²");
            assert_eq!(ModuloSquared::new(MOD_SQR).to_string(), "||7|²|²");

            // assert_eq!((-MOD_SQR).to_string(), "-|7|²");
            // assert_eq!((MOD_SQR + MOD_SQR).to_string(), "|7|² + |7|²");
            // assert_eq!((MOD_SQR - MOD_SQR).to_string(), "|7|² - |7|²");
            // assert_eq!((MOD_SQR * MOD_SQR).to_string(), "|7|² * |7|²");
            // assert_eq!((MOD_SQR / MOD_SQR).to_string(), "|7|² / |7|²");
            // assert_eq!(MOD_SQR.sqr().to_string(), "(|7|²)²");
        }
    }
}
