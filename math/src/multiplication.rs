use core::fmt::{self, Display};
use core::ops::{Add, Mul};

use crate::util::BinaryOperation;
use crate::{
    AlwaysConst, Apply, ComparisonTokens, Derivative, EvaluatedDisplayData, Function,
    FunctionDisplay, Output, RenameVariable, VariableNames, comparison, comparison_id,
    display_enum, impl_ops_for,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Multiplication<L, R> {
    pub left: L,
    pub right: R,
}

impl<L, R> Multiplication<L, R> {
    pub const fn new(left: L, right: R) -> Self {
        Multiplication { left, right }
    }
}

impl<L, R> Function for Multiplication<L, R> {}

impl<L, R, I, O> Output<I> for Multiplication<L, R>
where
    L: Output<I>,
    R: Output<I>,
    L::Output: Mul<R::Output, Output = O>,
{
    type Output = O;
}

impl<L, R, I> Apply<I> for Multiplication<L, R>
where
    L: Apply<I>,
    R: Apply<I>,
    L::Output: Mul<R::Output>,
    I: Clone,
{
    fn apply(this: &Self, input: I) -> Self::Output {
        this.binop_apply(input, Mul::mul)
    }
}

impl<L, R, const V: usize> Derivative<V> for Multiplication<L, R>
where
    // l'
    L: Derivative<V> + Clone,
    // r'
    R: Derivative<V> + Clone,
    // l' * r
    L::Derivative: Mul<R>,
    // l * r'
    L: Mul<R::Derivative>,
    // (l' * r) + (l * r')
    <L::Derivative as Mul<R>>::Output: Add<<L as Mul<R::Derivative>>::Output>,
{
    type Derivative =
        <<L::Derivative as Mul<R>>::Output as Add<<L as Mul<R::Derivative>>::Output>>::Output;

    fn derivative(this: &Self) -> Self::Derivative {
        let l = this.left.clone();
        let r = this.right.clone();

        let l_der = l.derivative();
        let r_der = r.derivative();

        /*
           mul(x) = l(x) . r(x)
           mul'(x) = l'(x).r(x) + l(x).r'(x)
        */

        l_der * r + l * r_der
    }
}

impl<L, R> ComparisonTokens for Multiplication<L, R>
where
    L: ComparisonTokens,
    R: ComparisonTokens,
{
    fn comparison_tokens(&self) -> impl Iterator<Item = comparison::Token<'_>> {
        self.binop_comparison_tokens(comparison_id!(Multiplication))
    }
}

impl<L, R> BinaryOperation<L, R> for Multiplication<L, R> {
    const SYMBOL: char = '*';

    fn left(&self) -> &L {
        &self.left
    }

    fn right(&self) -> &R {
        &self.right
    }
}

impl<L, R, const V: usize> RenameVariable<V> for Multiplication<L, R>
where
    L: RenameVariable<V>,
    R: RenameVariable<V>,
{
    type RenamedVar<N> = Multiplication<L::RenamedVar<N>, R::RenamedVar<N>>;

    fn rename_var_num<N: Clone>(this: &Self, name: N) -> Self::RenamedVar<N> {
        Multiplication::new(
            this.left.rename_var_num(name.clone()),
            this.right.rename_var_num(name),
        )
    }
}

impl<L, R> VariableNames for Multiplication<L, R>
where
    L: VariableNames,
    R: VariableNames,
{
    fn var_names(&self) -> impl Iterator<Item = (usize, &dyn Display)> {
        self.binop_var_names()
    }
}

impl<L, R> FunctionDisplay for Multiplication<L, R>
where
    L: FunctionDisplay,
    R: FunctionDisplay,
{
    const PRECEDENCE: u8 = 1;

    fn evaluate_display_data(&self) -> EvaluatedDisplayData<impl Display> {
        display_enum! {
            Case {
                Left,
                Right,
                NonSimplified,
            }
        }

        let left = self.left.evaluate_display_data();
        // TODO: single match for both evalutations?
        match left.always_const {
            // 0 * ? = 0
            Some(AlwaysConst::Zero) => left.map(Case::Left),
            // 1 * y = y
            Some(AlwaysConst::One) => self.right.evaluate_display_data().map(Case::Right),
            // x * ?
            None => {
                let right = self.right.evaluate_display_data();
                match right.always_const {
                    // x * 0 = 0
                    Some(AlwaysConst::Zero) => right.map(Case::Right),
                    // x * 1 = x
                    Some(AlwaysConst::One) => left.map(Case::Left),
                    // x * y
                    None => {
                        Self::binop_evaluated_display_data(left.display_data, right.display_data)
                            .map(Case::NonSimplified)
                    }
                }
            }
        }
    }
}

impl<L, R> Display for Multiplication<L, R>
where
    Self: FunctionDisplay,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_fmt(f)
    }
}

impl_ops_for!(Multiplication<L, R>);

#[macro_export]
macro_rules! impl_mul_for {
    ($name:ident $($token:tt)*) => {
        $crate::__parse_type_args!(impl_mul_for [@parsed $name] $($token)*);
    };

    (@parsed $name:ident [$($decl:tt)*] [$($use:tt)*]) => {
        impl< $($decl)* __F> core::ops::Mul<__F> for $name< $($use)* > {
            type Output = $crate::Multiplication<Self, __F>;

            fn mul(self, rhs: __F) -> Self::Output {
                $crate::Multiplication::new(self, rhs)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::FixedType;
    use crate::{
        Addition, Division, Multiplication, Negation, Squared, StaticOne, StaticZero, Subtraction,
        TypedConst, Variable,
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

    mod function_display {
        use super::*;

        #[test]
        fn simplification() {
            assert_eq!((ZERO * ZERO).to_string(), "0");
            assert_eq!((ZERO * ONE).to_string(), "0");
            assert_eq!((ZERO * VAR).to_string(), "0");

            assert_eq!((ONE * ZERO).to_string(), "0");
            assert_eq!((ONE * ONE).to_string(), "1");
            assert_eq!((ONE * VAR).to_string(), "x");

            assert_eq!((VAR * ZERO).to_string(), "0");
            assert_eq!((VAR * ONE).to_string(), "x");
            assert_eq!((VAR * VAR).to_string(), "x * x");
        }

        #[test]
        fn precedence() {
            assert_eq!((NEG * NEG).to_string(), "-7 * -7");
            assert_eq!((ADD * ADD).to_string(), "(7 + x) * (7 + x)");
            assert_eq!((SUB * SUB).to_string(), "(x - 7) * (x - 7)");
            assert_eq!((MUL * MUL).to_string(), "7 * x * (7 * x)");
            assert_eq!((DIV * DIV).to_string(), "x / 7 * (x / 7)");
            assert_eq!((SQR * SQR).to_string(), "7² * 7²");
        }
    }
}
