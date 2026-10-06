#![cfg_attr(not(test), no_std)]
#![feature(
    // TODO: clean-up unused/commented features (in the bins too).
    // auto_traits,
    const_trait_impl,
    const_cmp,
    generic_const_exprs,
    macro_metavar_expr,
    // negative_bounds,
    // negative_impls,
    // with_negative_coherence
)]
#![allow(incomplete_features)]

extern crate self as symbolic_calculus;

mod addition;
pub mod comparison;
mod division;
mod input;
mod multiplication;
mod named_const;
mod negation;
mod squared;
mod static_const;
mod subtraction;
mod typed_const;
pub mod util;
mod variable;

use core::fmt::{self, Display, Write as _};

pub use addition::*;
pub use division::*;
pub use input::*;
pub use multiplication::*;
pub use named_const::*;
pub use negation::*;
pub use squared::*;
pub use static_const::*;
pub use subtraction::*;
pub use typed_const::*;
pub use variable::*;

pub trait Function {
    fn apply<I>(&self, input: I) -> Self::Output
    where
        Self: Apply<I> + Sized,
    {
        Apply::apply(self, input)
    }

    fn derivative<const V: usize>(&self) -> Self::Derivative
    where
        Self: Derivative<V> + Sized,
    {
        Derivative::derivative(self)
    }

    fn rename_var<const V: usize, N, N2: Clone>(
        &self,
        _: &Variable<V, N>,
        name: N2,
    ) -> Self::RenamedVar<N2>
    where
        Self: RenameVariable<V>,
    {
        RenameVariable::rename_var_num(self, name)
    }

    fn rename_var_num<const V: usize, N: Clone>(&self, name: N) -> Self::RenamedVar<N>
    where
        Self: RenameVariable<V>,
    {
        RenameVariable::rename_var_num(self, name)
    }

    fn set_named_const_value<Id, V>(&self, value: V) -> Self::NamedConstValueSet<V>
    where
        Self: SetNamedConstValue<Id>,
        Id: NamedConstId,
        V: ValuedConst + Clone,
    {
        SetNamedConstValue::set_named_const_value(self, value)
    }

    fn as_fn<I>(&self) -> impl Fn(I) -> Self::Output
    where
        Self: Apply<I> + Sized,
    {
        |input| self.apply(input)
    }

    fn into_fn<I>(self) -> impl Fn(I) -> Self::Output
    where
        Self: Apply<I> + Sized,
    {
        move |input| self.apply(input)
    }

    fn as_two_vars_fn<I1, I2>(&self) -> impl Fn(I1, I2) -> Self::Output
    where
        Self: Apply<(I1, I2)> + Sized,
    {
        |v1, v2| self.apply((v1, v2))
    }

    fn into_two_vars_fn<I1, I2>(self) -> impl Fn(I1, I2) -> Self::Output
    where
        Self: Apply<(I1, I2)> + Sized,
    {
        move |v1, v2| self.apply((v1, v2))
    }
}

pub trait Output<I>: Function {
    type Output;
}

pub trait Apply<I>: Output<I> {
    fn apply(this: &Self, input: I) -> Self::Output;
}

pub trait Derivative<const V: usize>: Function {
    type Derivative;

    fn derivative(this: &Self) -> Self::Derivative;
}

pub trait RenameVariable<const V: usize>: Function {
    type RenamedVar<N>;

    fn rename_var_num<N: Clone>(this: &Self, name: N) -> Self::RenamedVar<N>;
}

pub trait VariableNames: Function {
    fn var_names(&self) -> impl Iterator<Item = (usize, &dyn Display)>;
}

pub trait SetNamedConstValue<Id>: Function {
    type NamedConstValueSet<V>;

    fn set_named_const_value<V: ValuedConst + Clone>(
        this: &Self,
        value: V,
    ) -> Self::NamedConstValueSet<V>;
}

pub trait Arity: Function {
    fn arity(&self) -> usize;
}

pub trait FunctionDisplay: Function {
    const PRECEDENCE: u8;

    fn evaluate_display_data(&self) -> EvaluatedDisplayData<impl Display>;

    fn display(&self) -> impl Display {
        self.evaluate_display_data().display_data.display
    }

    #[expect(clippy::missing_errors_doc)]
    fn display_fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display().fmt(f)
    }

    fn non_const_evaluated_display_data<D>(display: D) -> EvaluatedDisplayData<D> {
        EvaluatedDisplayData {
            always_const: None,
            display_data: DisplayData {
                precedence: Self::PRECEDENCE,
                display,
            },
        }
    }

    fn maybe_const_evaluated_display_data<D>(
        always_const: Option<AlwaysConst>,
        display: D,
    ) -> EvaluatedDisplayData<D> {
        EvaluatedDisplayData {
            always_const,
            display_data: DisplayData {
                precedence: Self::PRECEDENCE,
                display,
            },
        }
    }

    fn full_binop_evaluated_display_data(
        left: DisplayData<impl Display>,
        symbol: char,
        right: DisplayData<impl Display>,
    ) -> EvaluatedDisplayData<impl Display>
    where
        Self: Sized,
    {
        Self::non_const_evaluated_display_data(fmt::from_fn(move |f| {
            let left = left.left_operand_display::<Self>();
            let right = right.right_operand_display::<Self>();
            write!(f, "{} {symbol} {}", left, right,)
        }))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EvaluatedDisplayData<D> {
    pub always_const: Option<AlwaysConst>,
    pub display_data: DisplayData<D>,
}
impl<D: Display> EvaluatedDisplayData<D> {
    pub fn map<D2>(self, f: impl FnOnce(D) -> D2) -> EvaluatedDisplayData<D2> {
        EvaluatedDisplayData {
            always_const: self.always_const,
            display_data: self.display_data.map(f),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisplayData<D> {
    precedence: u8,
    display: D,
}
impl<D: Display> DisplayData<D> {
    pub fn map<D2>(self, f: impl FnOnce(D) -> D2) -> DisplayData<D2> {
        DisplayData {
            precedence: self.precedence,
            display: f(self.display),
        }
    }

    pub fn left_operand_display<Operation: FunctionDisplay>(&self) -> impl Display {
        self.operand_display(self.precedence < Operation::PRECEDENCE)
    }

    pub fn right_operand_display<Operation: FunctionDisplay>(&self) -> impl Display {
        self.operand_display(self.precedence <= Operation::PRECEDENCE)
    }

    fn operand_display(&self, parenthesized: bool) -> impl Display {
        fmt::from_fn(move |f| {
            if parenthesized {
                f.write_char('(')?;
            }
            self.display.fmt(f)?;
            if parenthesized {
                f.write_char(')')?;
            }
            Ok(())
        })
    }
}

pub trait ComparisonTokens: Function {
    fn comparison_tokens(&self) -> impl Iterator<Item = comparison::Token<'_>>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlwaysConst {
    Zero,
    One,
}

pub trait ValuedConst {}

#[macro_export]
macro_rules! impl_ops_for {
    ($name:ident $($token:tt)*) => {
        $crate::impl_neg_for!($name $($token)*);
        $crate::impl_add_for!($name $($token)*);
        $crate::impl_sub_for!($name $($token)*);
        $crate::impl_mul_for!($name $($token)*);
        $crate::impl_div_for!($name $($token)*);
        $crate::impl_sqr_for!($name $($token)*);
    };
}

/*
    Generates a list of declarations and a list of use from a list of generics.
    For instance, from `<const N: usize, T>`, it generates:
    - decl: `const N: usize, T`
    - use: `N, T`
*/
#[macro_export]
macro_rules! __parse_generics {
    // No generics.
    ([] $($rules:tt)+) => (
        $crate::__parse_generics!(@done [] [] $($rules)+);
    );

    // Generics found.
    ([< $($token:tt)+] $($rules:tt)+) => (
        $crate::__parse_generics!(@parse [$($token)+] [] [] $($rules)+);
    );

    // Generic const followed by comma.
    (@parse [const $id:ident : $ty:ty , $($token:tt)+] [$($decl:tt)*] [$($use:tt)*] $($rules:tt)+) => {
        $crate::__parse_generics!(@parse [$($token)+] [$($decl)* const $id: $ty,] [$($use)* $id,] $($rules)+);
    };

    // Generic const followed by '>'.
    (@parse [const $id:ident : $ty:ty >] [$($decl:tt)*] [$($use:tt)*] $($rules:tt)+) => {
        $crate::__parse_generics!(@done [$($decl)* const $id: $ty,] [$($use)* $id,] $($rules)+);
    };

    // Generic type followed by comma.
    (@parse [$id:ident , $($token:tt)+] [$($decl:tt)*] [$($use:tt)*] $($rules:tt)+) => {
        $crate::__parse_generics!(@parse [$($token)+] [$($decl)* $id,] [$($use)* $id,] $($rules)+);
    };

    // Generic type followed by '>'.
    (@parse [$id:ident >] [$($decl:tt)*] [$($use:tt)*] $($rules:tt)+) => {
        $crate::__parse_generics!(@done [$($decl)* $id,] [$($use)* $id,] $($rules)+);
    };

    // End of generics.
    (@parse [>] [$($decl:tt)*] [$($use:tt)*] $($rules:tt)+) => {
        $crate::__parse_generics!(@done [$($decl)*] [$($use)*] $($rules)+);
    };

    // Pass the tokens to the caller.
    (@done [$($decl:tt)*] [$($use:tt)*] $($rules:tt)+) => {
        macro_rules! __emit__ { $($rules)+ }
        __emit__!([$($decl)*] [$($use)*]);
    };
}
