#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub(crate) struct Wrapper<T>(pub(crate) T);

impl<T> Wrapper<T> {
    pub(crate) fn owned_to_owned(self) -> T {
        self.0
    }

    pub(crate) fn ref_to_ref(&self) -> &T {
        &self.0
    }

    pub(crate) fn ref_to_owned(&self) -> T
    where
        T: Copy,
    {
        self.0
    }
}
