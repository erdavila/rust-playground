use core::fmt::{self, Display};
use core::ops::Mul;

use crate::util::{OperationAssociativity, Sqr, UnaryOperation};
use crate::{
    AlwaysConst, Apply, ComparisonTokens, Derivative, EvaluatedDisplayData, Function,
    FunctionDisplay, Output, RenameVariable, StaticTwo, VariableNames, comparison, comparison_id,
    display_enum, impl_ops_for,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Squared<F>(pub F);

impl<F> Squared<F> {
    pub const fn new(func: F) -> Self {
        Squared(func)
    }
}

impl<F> Function for Squared<F> {}

impl<F, I> Output<I> for Squared<F>
where
    F: Output<I>,
    F::Output: Sqr,
{
    type Output = <F::Output as Sqr>::Output;
}

impl<F, I> Apply<I> for Squared<F>
where
    Self: Output<I>,
    F: Apply<I>,
    F::Output: Sqr<Output = Self::Output>,
{
    fn apply(this: &Self, input: I) -> Self::Output {
        this.0.apply(input).sqr()
    }
}

impl<F, const V: usize> Derivative<V> for Squared<F>
where
    F: Derivative<V> + Clone,
    StaticTwo<F>: Mul<F>,
    <StaticTwo<F> as Mul<F>>::Output: Mul<<F as Derivative<V>>::Derivative>,
{
    type Derivative =
        <<StaticTwo<F> as Mul<F>>::Output as Mul<<F as Derivative<V>>::Derivative>>::Output;

    fn derivative(this: &Self) -> Self::Derivative {
        /*
           f(x) = g(h(x))
                = h(x)²
           f'(x) = g'(h(x)).h'(x)
                 = 2.h(x).h'(x)

           g(x) = x²
           g'(x) = 2x
        */
        StaticTwo::new() * this.0.clone() * this.0.derivative::<V>()
    }
}

impl<F> ComparisonTokens for Squared<F>
where
    F: ComparisonTokens,
{
    fn comparison_tokens(&self) -> impl Iterator<Item = comparison::Token<'_>> {
        comparison::TokensBuilder::new(comparison_id!(Squared))
            .from_child(&self.0)
            .build()
    }
}

impl<F, const V: usize> RenameVariable<V> for Squared<F>
where
    F: RenameVariable<V>,
{
    type RenamedVar<N> = Squared<F::RenamedVar<N>>;

    fn rename_var_num<N: Clone>(this: &Self, name: N) -> Self::RenamedVar<N> {
        Squared::new(this.0.rename_var_num(name))
    }
}

impl<F> VariableNames for Squared<F>
where
    F: VariableNames,
{
    fn var_names(&self) -> impl Iterator<Item = (usize, &dyn Display)> {
        self.0.var_names()
    }
}

impl<F> FunctionDisplay for Squared<F>
where
    F: FunctionDisplay,
{
    const PRECEDENCE: u8 = 2;

    fn evaluate_display_data(&self) -> EvaluatedDisplayData<impl Display> {
        display_enum! {
            Case {
                Operand,
                NonSimplified,
            }
        }

        let func = self.0.evaluate_display_data();
        match func.always_const {
            // 0² = 0, 1² = 1
            Some(AlwaysConst::Zero | AlwaysConst::One) => func.map(Case::Operand),
            // x²
            None => Self::unop_evaluated_display_data(func.display_data).map(Case::NonSimplified),
        }
    }
}

impl<F> UnaryOperation<F> for Squared<F> {
    const DISPLAY_PREFIX: &str = "";
    const DISPLAY_SUFFIX: &str = "²";
    const ASSOCIATIVITY: OperationAssociativity = OperationAssociativity::Right;
}

impl<F> Display for Squared<F>
where
    Self: FunctionDisplay,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_fmt(f)
    }
}

impl_ops_for!(Squared<F>);

#[macro_export]
macro_rules! impl_sqr_for {
    ($name:ident $($token:tt)*) => {
        $crate::__parse_type_args!(impl_sqr_for [@parsed $name] $($token)*);
    };

    (@parsed $name:ident [$($decl:tt)*] [$($use:tt)*]) => {
        impl< $($decl)*> $crate::util::Sqr for $name< $($use)* > {
            type Output = $crate::Squared<Self>;

            fn sqr(self) -> Self::Output {
                $crate::Squared(self)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    mod function_display {
        use super::*;
        use crate::util::FixedType;
        use crate::{
            Addition, Division, Multiplication, Negation, Squared, StaticOne, StaticZero,
            Subtraction, TypedConst, Variable,
        };

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

        #[test]
        fn simplification() {
            assert_eq!(ZERO.sqr().to_string(), "0");
            assert_eq!(ONE.sqr().to_string(), "1");
            assert_eq!(VAR.sqr().to_string(), "x²");
        }

        #[test]
        fn precedence() {
            assert_eq!(VAR.sqr().to_string(), "x²");
            assert_eq!(NEG.sqr().to_string(), "(-7)²");
            assert_eq!(ADD.sqr().to_string(), "(7 + x)²");
            assert_eq!(SUB.sqr().to_string(), "(x - 7)²");
            assert_eq!(MUL.sqr().to_string(), "(7 * x)²");
            assert_eq!(DIV.sqr().to_string(), "(x / 7)²");
            assert_eq!(SQR.sqr().to_string(), "(7²)²");
        }
    }
}
