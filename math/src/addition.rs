use core::fmt::{self, Display};
use core::ops::Add;

use crate::util::BinaryOperation;
use crate::{
    AlwaysConst, Apply, ComparisonTokens, Derivative, EvaluatedDisplayData, Function,
    FunctionDisplay, Output, RenameVariable, StaticTwo, VariableNames, comparison, comparison_id,
    display_enum, impl_ops_for,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Addition<L, R> {
    pub left: L,
    pub right: R,
}

impl<L, R> Addition<L, R> {
    pub const fn new(left: L, right: R) -> Self {
        Addition { left, right }
    }
}

impl<L, R> Function for Addition<L, R> {}

impl<L, R, I> Output<I> for Addition<L, R>
where
    L: Output<I>,
    R: Output<I>,
    L::Output: Add<R::Output>,
{
    type Output = <L::Output as Add<R::Output>>::Output;
}

impl<L, R, I> Apply<I> for Addition<L, R>
where
    L: Apply<I>,
    R: Apply<I>,
    L::Output: Add<R::Output>,
    I: Clone,
{
    fn apply(this: &Self, input: I) -> Self::Output {
        this.binop_apply(input, Add::add)
    }
}

impl<L, R, const V: usize> Derivative<V> for Addition<L, R>
where
    // l'
    L: Derivative<V>,
    // r'
    R: Derivative<V>,
    // l' + r'
    L::Derivative: Add<R::Derivative>,
{
    type Derivative = <L::Derivative as Add<R::Derivative>>::Output;

    fn derivative(this: &Self) -> Self::Derivative {
        /*
           add(x) = l(x) + r(x)
           add'(x) = l'(x) + r'(x)
        */
        Derivative::derivative(&this.left) + Derivative::derivative(&this.right)
    }
}

impl<L, R> ComparisonTokens for Addition<L, R>
where
    L: ComparisonTokens,
    R: ComparisonTokens,
{
    fn comparison_tokens(&self) -> impl Iterator<Item = comparison::Token<'_>> {
        self.binop_comparison_tokens(comparison_id!(Addition))
    }
}

impl<L, R> BinaryOperation<L, R> for Addition<L, R> {
    const SYMBOL: char = '+';

    fn left(&self) -> &L {
        &self.left
    }

    fn right(&self) -> &R {
        &self.right
    }
}

impl<L, R, const V: usize> RenameVariable<V> for Addition<L, R>
where
    L: RenameVariable<V>,
    R: RenameVariable<V>,
{
    type RenamedVar<N> = Addition<L::RenamedVar<N>, R::RenamedVar<N>>;

    fn rename_var_num<N>(this: &Self, name: N) -> Self::RenamedVar<N>
    where
        N: Clone,
    {
        Addition::new(
            RenameVariable::rename_var_num(&this.left, name.clone()),
            RenameVariable::rename_var_num(&this.right, name.clone()),
        )
    }
}

impl<L, R> VariableNames for Addition<L, R>
where
    L: VariableNames,
    R: VariableNames,
{
    fn var_names(&self) -> impl Iterator<Item = (usize, &dyn Display)> {
        self.binop_var_names()
    }
}

impl<L, R> FunctionDisplay for Addition<L, R>
where
    L: FunctionDisplay,
    R: FunctionDisplay,
{
    const PRECEDENCE: u8 = 0;

    fn evaluate_display_data(&self) -> EvaluatedDisplayData<impl Display> {
        display_enum! {
            Case {
                Two,
                Left,
                Right,
                Both,
            }
        };

        let left = self.left.evaluate_display_data();
        // TODO: single match for both evalutations?
        match left.always_const {
            // 0 + x = x
            Some(AlwaysConst::Zero) => self.right.evaluate_display_data().map(Case::Right),
            // 1 + ?
            Some(AlwaysConst::One) => {
                let right = self.right.evaluate_display_data();
                // TODO: match needed?
                match right.always_const {
                    // 1 + 0 = 1
                    Some(AlwaysConst::Zero) => left.map(Case::Left),
                    // 1 + 1 = 2
                    Some(AlwaysConst::One) => StaticTwo::<Self>::new()
                        .evaluate_display_data()
                        .map(Case::Two),
                    // 1 + x
                    None => {
                        Self::binop_evaluated_display_data(left.display_data, right.display_data)
                            .map(Case::Both)
                    }
                }
            }
            // x + ?
            None => {
                let right = self.right.evaluate_display_data();
                // TODO: match needed?
                match right.always_const {
                    // x + 0 = x
                    Some(AlwaysConst::Zero) => left.map(Case::Left),
                    // x + 1, x + y
                    Some(AlwaysConst::One) | None => {
                        Self::binop_evaluated_display_data(left.display_data, right.display_data)
                            .map(Case::Both)
                    }
                }
            }
        }
    }
}

impl<L, R> Display for Addition<L, R>
where
    Self: FunctionDisplay,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_fmt(f)
    }
}

impl_ops_for!(Addition<L, R>);

#[macro_export]
macro_rules! impl_add_for {
    ($name:ident $($token:tt)*) => {
        $crate::__parse_type_args!(impl_add_for [@parsed $name] $($token)*);
    };

    (@parsed $name:ident [$($decl:tt)*] [$($use:tt)*]) => {
        impl< $($decl)* __F> core::ops::Add<__F> for $name< $($use)* > {
            type Output = $crate::Addition<Self, __F>;

            fn add(self, rhs: __F) -> Self::Output {
                $crate::Addition::new(self, rhs)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::FixedType;
    use crate::{
        Division, Multiplication, Negation, Squared, StaticOne, StaticZero, Subtraction,
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
            assert_eq!((ZERO + ZERO).to_string(), "0");
            assert_eq!((ZERO + ONE).to_string(), "1");
            assert_eq!((ZERO + VAR).to_string(), "x");

            assert_eq!((ONE + ZERO).to_string(), "1");
            assert_eq!((ONE + ONE).to_string(), "2");
            assert_eq!((ONE + VAR).to_string(), "1 + x");

            assert_eq!((VAR + ZERO).to_string(), "x");
            assert_eq!((VAR + ONE).to_string(), "x + 1");
            assert_eq!((VAR + VAR).to_string(), "x + x");
        }

        #[test]
        fn precedence() {
            assert_eq!((NEG + NEG).to_string(), "-7 + -7");
            assert_eq!((ADD + ADD).to_string(), "7 + x + (7 + x)");
            assert_eq!((SUB + SUB).to_string(), "x - 7 + (x - 7)");
            assert_eq!((MUL + MUL).to_string(), "7 * x + 7 * x");
            assert_eq!((DIV + DIV).to_string(), "x / 7 + x / 7");
            assert_eq!((SQR + SQR).to_string(), "7² + 7²");
        }
    }
}
