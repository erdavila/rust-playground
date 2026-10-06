#[doc(hidden)]
mod internal;

use core::fmt::{Debug, Display};
use core::marker::PhantomData;

use symbolic_calculus_macros::{
    Arity, Derivative, Display, Function, RenameVariable, VariableNames,
};

use crate::named_const::internal::{DispatchSetNamedConstValue, is_same};
use crate::{
    ComparisonTokens, EvaluatedDisplayData, FunctionDisplay, SetNamedConstValue, StaticZero,
    ValuedConst, comparison, impl_ops_for,
};

pub trait NamedConstId: Copy + Eq {
    type Name: Display;
    const NAME: Self::Name;
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Function,
    Derivative,
    RenameVariable,
    VariableNames,
    Arity,
    Display,
)]
#[function(const, type_function = Self)]
pub struct NamedConst<Id>(PhantomData<Id>);

impl<Id> NamedConst<Id> {
    #[must_use]
    pub fn new() -> Self {
        NamedConst(PhantomData)
    }
}

impl<Id> Default for NamedConst<Id> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Id: NamedConstId> From<Id> for NamedConst<Id> {
    fn from(_value: Id) -> Self {
        NamedConst::new()
    }
}

impl<Id: 'static, Id2: 'static> SetNamedConstValue<Id2> for NamedConst<Id>
where
    [(); is_same::<Id, Id2>() as usize]:,
    Self: DispatchSetNamedConstValue<{ is_same::<Id, Id2>() }>,
{
    type NamedConstValueSet<V> =
        <Self as DispatchSetNamedConstValue<{ is_same::<Id, Id2>() }>>::Out<V>;

    fn set_named_const_value<V: ValuedConst + Clone>(
        this: &Self,
        value: V,
    ) -> Self::NamedConstValueSet<V> {
        <Self as DispatchSetNamedConstValue< { is_same::<Id, Id2>() } >>::displatch_set_named_const_value(this, value)
    }
}

impl<Id: NamedConstId> ComparisonTokens for NamedConst<Id> {
    fn comparison_tokens(&self) -> impl Iterator<Item = comparison::Token<'_>> {
        [todo!("How to include `Id` in the tokens?")].into_iter()
        // comparison::TokensBuilder::new(comparison_id!(TypedConst))
        //     .value(&Id::NAME)
        //     .build()
    }
}

impl<Id: NamedConstId> FunctionDisplay for NamedConst<Id> {
    const PRECEDENCE: u8 = 2;

    fn evaluated_display_data(&self) -> EvaluatedDisplayData<impl Display> {
        Self::non_const_evaluated_display_data(Id::NAME)
    }
}

impl_ops_for!(NamedConst<Id>);

#[macro_export]
// TODO: what exactly this macro should do?
macro_rules! define_named_const {
    ($name:ident : $type:ident) => {
        #[allow(non_camel_case_types)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        struct $type;
        impl NamedConstId for $type {
            type Name = &'static str;
            const NAME: Self::Name = stringify!($name);
        }
        let $name = $crate::NamedConst::<$type>::new();
    };
}
