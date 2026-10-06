use core::fmt::{self, Display};
use core::ops::Neg;

use crate::util::{OperationAssociativity, UnaryOperation};
use crate::{
    AlwaysConst, Apply, Derivative, EvaluatedDisplayData, Function, FunctionDisplay, Output,
    RenameVariable, StaticZero, VariableNames, display_enum, impl_ops_for,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Negation<F>(pub F);

impl<F> Negation<F> {
    pub const fn new(func: F) -> Self {
        Negation(func)
    }
}

impl<F> Function for Negation<F> {}

impl<F, I> Output<I> for Negation<F>
where
    F: Output<I>,
    F::Output: Neg,
{
    type Output = <F::Output as Neg>::Output;
}

impl<F, I> Apply<I> for Negation<F>
where
    Self: Output<I>,
    F: Apply<I>,
    F::Output: Neg<Output = Self::Output>,
{
    fn apply(this: &Self, input: I) -> Self::Output {
        -this.0.apply(input)
    }
}

impl<F, const V: usize> Derivative<V> for Negation<F>
where
    F: Derivative<V>,
    F::Derivative: Neg,
{
    type Derivative = <F::Derivative as Neg>::Output;

    fn derivative(this: &Self) -> Self::Derivative {
        /*
           neg(x) = -f(x)
                  = g(f(x))
           neg'(x) = g'(f(x)).f'(x)
                   = -f'(x)

           g(x) = -x
           g'(x) = -1
        */
        -this.0.derivative()
    }
}

impl<F, const V: usize> RenameVariable<V> for Negation<F>
where
    F: RenameVariable<V>,
{
    type RenamedVar<N> = Negation<F::RenamedVar<N>>;

    fn rename_var_num<N: Clone>(this: &Self, name: N) -> Self::RenamedVar<N> {
        Negation::new(this.0.rename_var_num(name))
    }
}

impl<F> VariableNames for Negation<F>
where
    F: VariableNames,
{
    fn var_names(&self) -> impl Iterator<Item = (usize, &dyn Display)> {
        self.0.var_names()
    }
}

impl<F> FunctionDisplay for Negation<F>
where
    F: FunctionDisplay,
{
    const PRECEDENCE: u8 = 2;

    fn evaluate_display_data(&self) -> EvaluatedDisplayData<impl Display> {
        display_enum! {
            Case {
                Zero,
                NonZero
            }
        }

        let func = self.0.evaluate_display_data();
        match func.always_const {
            // -0 = 0
            Some(AlwaysConst::Zero) => StaticZero::<Self>::new()
                .evaluate_display_data()
                .map(Case::Zero),
            // -1, -x
            Some(AlwaysConst::One) | None => {
                Self::unop_evaluated_display_data(func.display_data).map(Case::NonZero)
            }
        }
    }
}

impl<F> UnaryOperation<F> for Negation<F> {
    const DISPLAY_PREFIX: &str = "-";
    const DISPLAY_SUFFIX: &str = "";
    const ASSOCIATIVITY: OperationAssociativity = OperationAssociativity::Left;
}

impl<F> Display for Negation<F>
where
    Self: FunctionDisplay,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_fmt(f)
    }
}

impl_ops_for!(Negation<F>);

#[macro_export]
macro_rules! impl_neg_for {
    ($name:ident $($token:tt)*) => {
        $crate::__parse_type_args!(impl_neg_for [@parsed $name] $($token)*);
    };

    (@parsed $name:ident [$($decl:tt)*] [$($use:tt)*]) => {
        impl< $($decl)* > core::ops::Neg for $name< $($use)* > {
            type Output = $crate::Negation<Self>;

            fn neg(self) -> Self::Output {
                $crate::Negation::new(self)
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
            Addition, Division, Multiplication, Squared, StaticOne, Subtraction, TypedConst,
            Variable,
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
            assert_eq!((-ZERO).to_string(), "0");
            assert_eq!((-ONE).to_string(), "-1");
            assert_eq!((-VAR).to_string(), "-x");
        }

        #[test]
        fn precedence() {
            assert_eq!((-VAR).to_string(), "-x");
            assert_eq!((-NEG).to_string(), "--7");
            assert_eq!((-ADD).to_string(), "-(7 + x)");
            assert_eq!((-SUB).to_string(), "-(x - 7)");
            assert_eq!((-MUL).to_string(), "-(7 * x)");
            assert_eq!((-DIV).to_string(), "-(x / 7)");
            assert_eq!((-SQR).to_string(), "-7²");
        }
    }
}
