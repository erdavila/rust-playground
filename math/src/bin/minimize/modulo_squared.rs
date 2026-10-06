use std::fmt::{self, Display};
use std::ops::Add;

use math::util::{OperationAssociativity, Sqr, UnaryOperation};
use math::{
    AlwaysConst, Apply, ComparisonTokens, EvaluatedDisplayData, Function, FunctionDisplay, Output,
    RenameVariable, VariableNames, comparison, comparison_id, display_enum, impl_ops_for,
};

use crate::complex::Complex;
use crate::real::Real;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ModuloSquared<F>(pub F);

impl<F> ModuloSquared<F> {
    pub const fn new(func: F) -> Self {
        ModuloSquared(func)
    }
}

impl<F> Function for ModuloSquared<F> {}

impl<F, I, O> Output<I> for ModuloSquared<F>
where
    F: Output<I>,
    <F as Output<I>>::Output: ModSqr<Output = O>,
{
    type Output = O;
}

impl<F, I> Apply<I> for ModuloSquared<F>
where
    F: Apply<I>,
    <F as Output<I>>::Output: ModSqr,
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

impl<F> ComparisonTokens for ModuloSquared<F>
where
    F: ComparisonTokens,
{
    fn comparison_tokens(&self) -> impl Iterator<Item = comparison::Token<'_>> {
        comparison::TokensBuilder::new(comparison_id!(ModuloSquared))
            .from_child(&self.0)
            .build()
    }
}

impl<F, const V: usize> RenameVariable<V> for ModuloSquared<F>
where
    F: RenameVariable<V>,
{
    type RenamedVar<N> = ModuloSquared<F::RenamedVar<N>>;

    fn rename_var_num<N: Clone>(this: &Self, name: N) -> Self::RenamedVar<N> {
        ModuloSquared::new(this.0.rename_var_num(name))
    }
}

impl<F> VariableNames for ModuloSquared<F>
where
    F: VariableNames,
{
    fn var_names(&self) -> impl Iterator<Item = (usize, &dyn Display)> {
        self.0.var_names()
    }
}

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
    const ASSOCIATIVITY: OperationAssociativity = OperationAssociativity::Left;
}

impl<F> Display for ModuloSquared<F>
where
    Self: FunctionDisplay,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_fmt(f)
    }
}

impl_ops_for!(ModuloSquared<F>);

pub(crate) trait ModSqr {
    type Output;
    fn mod_sqr(self) -> Self::Output;
}
impl<R: Real> ModSqr for R
where
    R: Sqr + Clone,
{
    type Output = <R as Sqr>::Output;

    fn mod_sqr(self) -> Self::Output {
        self.sqr()
    }
}
impl<R> ModSqr for Complex<R>
where
    R: Sqr,
    <R as Sqr>::Output: Add,
{
    type Output = <<R as Sqr>::Output as Add>::Output;

    fn mod_sqr(self) -> Self::Output {
        self.real.sqr() + self.imag.sqr()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod function_display {
        use math::util::FixedType;
        use math::{
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
