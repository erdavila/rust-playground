use core::ops::Bound;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Endpoint<K> {
    Open(K),
    Closed(K),
    Infinity,
}

impl<K> Endpoint<K> {
    pub fn map<U>(self, mut f: impl FnMut(K) -> U) -> Endpoint<U> {
        match self {
            Endpoint::Open(k) => Endpoint::Open(f(k)),
            Endpoint::Closed(k) => Endpoint::Closed(f(k)),
            Endpoint::Infinity => Endpoint::Infinity,
        }
    }

    pub fn as_ref(&self) -> Endpoint<&K> {
        match self {
            Endpoint::Open(k) => Endpoint::Open(k),
            Endpoint::Closed(k) => Endpoint::Closed(k),
            Endpoint::Infinity => Endpoint::Infinity,
        }
    }

    pub fn as_mut(&mut self) -> Endpoint<&mut K> {
        match self {
            Endpoint::Open(k) => Endpoint::Open(k),
            Endpoint::Closed(k) => Endpoint::Closed(k),
            Endpoint::Infinity => Endpoint::Infinity,
        }
    }

    pub fn into_range_bound(self) -> Bound<K> {
        match self {
            Endpoint::Open(k) => Bound::Excluded(k),
            Endpoint::Closed(k) => Bound::Included(k),
            Endpoint::Infinity => Bound::Unbounded,
        }
    }

    pub(crate) fn opposite(&self) -> Option<Endpoint<&K>> {
        match self {
            Endpoint::Open(k) => Some(Endpoint::Closed(k)),
            Endpoint::Closed(k) => Some(Endpoint::Open(k)),
            Endpoint::Infinity => None,
        }
    }
}

impl<K> Endpoint<&K> {
    #[must_use]
    pub fn cloned(self) -> Endpoint<K>
    where
        K: Clone,
    {
        self.map(K::clone)
    }

    #[must_use]
    pub fn copied(self) -> Endpoint<K>
    where
        K: Copy,
    {
        self.map(|k| *k)
    }
}

impl<K> From<Bound<K>> for Endpoint<K> {
    fn from(value: Bound<K>) -> Self {
        match value {
            Bound::Included(k) => Endpoint::Closed(k),
            Bound::Excluded(k) => Endpoint::Open(k),
            Bound::Unbounded => Endpoint::Infinity,
        }
    }
}

impl<K> From<Endpoint<K>> for Bound<K> {
    fn from(value: Endpoint<K>) -> Self {
        value.into_range_bound()
    }
}
