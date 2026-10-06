use crate::{StaticOne, StaticZero, Variable};

pub trait DerivativeDispatch<const IS_SAME: bool> {
    type Der;

    fn dispatch_derivative() -> Self::Der;
}
impl<T> DerivativeDispatch<true> for T {
    type Der = StaticOne<Self>;

    fn dispatch_derivative() -> Self::Der {
        StaticOne::new()
    }
}
impl<T> DerivativeDispatch<false> for T {
    type Der = StaticZero<Self>;

    fn dispatch_derivative() -> Self::Der {
        StaticZero::new()
    }
}

pub trait RenameVariableDispatch<const IS_SAME: bool> {
    type Ren<N>;

    // TODO: change from `&self` to `this: &Self`
    fn dispatch_rename_var<T>(&self, name: T) -> Self::Ren<T>;
}
impl<const V: usize, N> RenameVariableDispatch<true> for Variable<V, N> {
    type Ren<N2> = Variable<V, N2>;

    fn dispatch_rename_var<N2>(&self, name: N2) -> Self::Ren<N2> {
        Variable::with_name(name)
    }
}
impl<const V: usize, N> RenameVariableDispatch<false> for Variable<V, N>
where
    Self: Clone,
{
    type Ren<N2> = Self;

    fn dispatch_rename_var<N2>(&self, _name: N2) -> Self::Ren<N2> {
        self.clone()
    }
}
