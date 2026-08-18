use core::convert::Infallible;
use core::ops::{Index, IndexMut};

pub use enum_indexed_macro::IndexedStruct;

pub mod iter;

pub trait EnumIndexed<T, const N: usize>:
    Sized + Index<Self::Enum, Output = T> + IndexMut<Self::Enum>
{
    type Enum: Copy;
    type Map<U>: EnumIndexed<U, N, Enum = Self::Enum>;

    #[must_use]
    fn from_fn(mut f: impl FnMut(Self::Enum) -> T) -> Self {
        let Ok(s) = Self::try_from_fn::<Infallible>(|e| Ok(f(e)));
        s
    }

    #[expect(clippy::missing_errors_doc)]
    fn try_from_fn<E>(f: impl FnMut(Self::Enum) -> Result<T, E>) -> Result<Self, E>;

    #[must_use]
    fn map<U>(self, mut f: impl FnMut(T) -> U) -> Self::Map<U> {
        self.map_enumerated(|_, value| f(value))
    }

    #[must_use]
    fn map_enumerated<U>(self, mut f: impl FnMut(Self::Enum, T) -> U) -> Self::Map<U> {
        let Ok(s) =
            self.try_map_enumerated::<U, Infallible>(|variant, value| Ok(f(variant, value)));
        s
    }

    #[expect(clippy::missing_errors_doc)]
    fn try_map<U, E>(self, mut f: impl FnMut(T) -> Result<U, E>) -> Result<Self::Map<U>, E> {
        self.try_map_enumerated(|_, value| f(value))
    }

    #[expect(clippy::missing_errors_doc)]
    fn try_map_enumerated<U, E>(
        self,
        f: impl FnMut(Self::Enum, T) -> Result<U, E>,
    ) -> Result<Self::Map<U>, E>;

    #[must_use]
    fn iter(&self) -> iter::Iter<'_, T, N, Self> {
        iter::Iter::new(self)
    }

    #[must_use]
    fn iter_mut(&mut self) -> iter::IterMut<'_, T, N, Self> {
        iter::IterMut::new(self)
    }

    #[must_use]
    fn values<'a>(&'a self) -> iter::Values<'a, T, N, Self>
    where
        T: 'a,
    {
        iter::Values::new(self)
    }

    #[must_use]
    fn values_mut<'a>(&'a mut self) -> iter::ValuesMut<'a, T, N, Self>
    where
        T: 'a,
    {
        iter::ValuesMut::new(self)
    }

    #[must_use]
    fn into_values(self) -> iter::IntoValues<T, N> {
        iter::IntoValues::new(self)
    }

    #[must_use]
    fn as_ref(&self) -> Self::Map<&T>;

    #[must_use]
    fn as_mut(&mut self) -> Self::Map<&mut T>;

    fn swap(&mut self, x: Self::Enum, y: Self::Enum);

    #[must_use]
    fn into_array(self) -> [T; N] {
        self.into_array_enumerated().map(|(_, value)| value)
    }

    #[must_use]
    fn into_array_enumerated(self) -> [(Self::Enum, T); N];

    #[must_use]
    fn variant_from_index(index: usize) -> Self::Enum;
}
