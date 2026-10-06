mod convert;

use core::fmt::{self, Display};
use core::marker::PhantomData;
use core::ops::Mul;
use std::cmp::Ordering;

pub use convert::*;

use crate::{
    Apply, ComparisonTokens, DisplayData, EvaluatedDisplayData, Function, FunctionDisplay, Output,
    VariableNames, comparison, impl_ops_for,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationAssociativity {
    Left,
    Right,
}

pub trait UnaryOperation<F> {
    const DISPLAY_PREFIX: &str;
    const DISPLAY_SUFFIX: &str;
    const ASSOCIATIVITY: OperationAssociativity;

    fn unop_evaluated_display_data(
        func: DisplayData<impl Display>,
    ) -> EvaluatedDisplayData<impl Display>
    where
        Self: FunctionDisplay + Sized,
    {
        EvaluatedDisplayData {
            always_const: None,
            display_data: DisplayData {
                precedence: Self::PRECEDENCE,
                display: fmt::from_fn(move |f| {
                    f.write_str(Self::DISPLAY_PREFIX)?;
                    match Self::ASSOCIATIVITY {
                        OperationAssociativity::Left => {
                            func.left_operand_display::<Self>().fmt(f)?;
                        }
                        OperationAssociativity::Right => {
                            func.right_operand_display::<Self>().fmt(f)?;
                        }
                    }
                    f.write_str(Self::DISPLAY_SUFFIX)?;
                    Ok(())
                }),
            },
        }
    }
}

// TODO: turn into macros?
pub trait BinaryOperation<L, R> {
    const SYMBOL: char;

    fn left(&self) -> &L;
    fn right(&self) -> &R;

    fn binop_apply<I>(
        &self,
        input: I,
        op: impl FnOnce(L::Output, R::Output) -> Self::Output,
    ) -> Self::Output
    where
        Self: Output<I>,
        L: Apply<I>,
        R: Apply<I>,
        I: Clone,
    {
        let l = self.left().apply(input.clone());
        let r = self.right().apply(input);
        op(l, r)
    }

    fn binop_comparison_tokens<'a>(
        &'a self,
        id: comparison::Id,
    ) -> impl Iterator<Item = comparison::Token<'a>>
    where
        L: ComparisonTokens + 'a,
        R: ComparisonTokens + 'a,
    {
        comparison::TokensBuilder::new(id)
            .from_child(self.left())
            .from_child(self.right())
            .build()
    }

    fn binop_evaluated_display_data(
        left: DisplayData<impl Display>,
        right: DisplayData<impl Display>,
    ) -> EvaluatedDisplayData<impl Display>
    where
        Self: FunctionDisplay + Sized,
    {
        <Self as FunctionDisplay>::non_const_evaluated_display_data(fmt::from_fn(move |f| {
            write!(
                f,
                "{} {} {}",
                left.left_operand_display::<Self>(),
                Self::SYMBOL,
                right.right_operand_display::<Self>(),
            )
        }))
    }

    fn binop_var_names<'a>(&'a self) -> impl Iterator<Item = (usize, &'a dyn Display)>
    where
        L: VariableNames + 'a,
        R: VariableNames + 'a,
    {
        MergeVarNames::new(self.left().var_names(), self.right().var_names())
    }
}

#[derive(Clone)]
pub struct MergeVarNames<'a, IL, IR> {
    iter_left: IL,
    iter_right: IR,
    next_left: Option<(usize, &'a dyn Display)>,
    next_right: Option<(usize, &'a dyn Display)>,
}
impl<IL, IR> MergeVarNames<'_, IL, IR> {
    pub const fn new(iter_left: IL, iter_right: IR) -> Self {
        MergeVarNames {
            iter_left,
            iter_right,
            next_left: None,
            next_right: None,
        }
    }
}
impl<'a, IL, IR> Iterator for MergeVarNames<'a, IL, IR>
where
    IL: Iterator<Item = (usize, &'a dyn Display)>,
    IR: Iterator<Item = (usize, &'a dyn Display)>,
{
    type Item = (usize, &'a dyn Display);

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_left.is_none() {
            self.next_left = self.iter_left.next();
        }
        if self.next_right.is_none() {
            self.next_right = self.iter_right.next();
        }

        match (self.next_left, self.next_right) {
            (Some((l_num, _)), Some((r_num, _))) => match l_num.cmp(&r_num) {
                Ordering::Less => self.next_left.take(),
                Ordering::Equal => {
                    self.next_left = None;
                    self.next_right.take()
                }
                Ordering::Greater => self.next_right.take(),
            },
            (Some(_), None) => self.next_left.take(),
            (None, Some(_)) => self.next_right.take(),
            (None, None) => None,
        }
    }
}

#[macro_export]
macro_rules! display_enum {
    ($name:ident { $($variant:ident),* $(,)? }) => {
        enum $name<$($variant),*> {
            $(
                $variant($variant),
            )*
        }
        impl<$($variant: Display),*> core::fmt::Display for $name<$($variant),*> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                match self {
                    $(
                        $name::$variant(x) => x.fmt(f),
                    )*
                }
            }
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FixedType<T>(PhantomData<T>);
impl<T> Function for FixedType<T> {}
impl<T, I> Output<I> for FixedType<T> {
    type Output = T;
}
impl_ops_for!(FixedType<T>);

pub trait Sqr {
    type Output;

    fn sqr(self) -> Self::Output;
}
impl Sqr for f64 {
    type Output = <f64 as Mul>::Output;

    fn sqr(self) -> Self::Output {
        self * self
    }
}
