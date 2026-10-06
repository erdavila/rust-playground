use core::any::TypeId;

use crate::NamedConst;

#[must_use]
pub const fn is_same<U: 'static, V: 'static>() -> bool {
    TypeId::of::<U>() == TypeId::of::<V>()
}

pub trait DispatchSetNamedConstValue<const IS_SAME: bool> {
    type Out<V>;

    fn displatch_set_named_const_value<V>(this: &Self, value: V) -> Self::Out<V>;
}
impl<Id: 'static> DispatchSetNamedConstValue<true> for NamedConst<Id> {
    type Out<V> = V;

    fn displatch_set_named_const_value<V>(_this: &Self, value: V) -> Self::Out<V> {
        value
    }
}
impl<Id: 'static> DispatchSetNamedConstValue<false> for NamedConst<Id>
where
    Self: Clone,
{
    type Out<V> = Self;

    fn displatch_set_named_const_value<V>(this: &Self, _value: V) -> Self::Out<V> {
        this.clone()
    }
}
