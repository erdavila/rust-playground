use core::array;
use core::iter::FusedIterator;

use crate::indexed_struct::EnumIndexed;
use crate::indexed_struct::iter::indexes::Indexes;

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct Iter<'a, T, const N: usize, S: EnumIndexed<T, N>> {
    indexed_struct: &'a S,
    indexes: Indexes<T, N, S>,
}

impl<'a, T, const N: usize, S: EnumIndexed<T, N>> Iter<'a, T, N, S> {
    pub fn new(indexed_struct: &'a S) -> Self {
        Iter {
            indexed_struct,
            indexes: Indexes::new(),
        }
    }
}

impl<'a, T: 'a, const N: usize, S: EnumIndexed<T, N>> Iterator for Iter<'a, T, N, S> {
    type Item = (S::Enum, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        self.indexes
            .next_front()
            .map(|variant| (variant, &self.indexed_struct[variant]))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.len();
        (size, Some(size))
    }
}

impl<'a, T: 'a, const N: usize, S: EnumIndexed<T, N>> DoubleEndedIterator for Iter<'a, T, N, S> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.indexes
            .next_back()
            .map(|variant| (variant, &self.indexed_struct[variant]))
    }
}

impl<'a, T: 'a, const N: usize, S: EnumIndexed<T, N>> ExactSizeIterator for Iter<'a, T, N, S> {
    fn len(&self) -> usize {
        self.indexes.remaining_count()
    }
}

impl<'a, T: 'a, const N: usize, S: EnumIndexed<T, N>> FusedIterator for Iter<'a, T, N, S> {}

////////////////////////////////////////////////////////////////////////////////

pub struct IterMut<'a, T, const N: usize, S: EnumIndexed<T, N>> {
    indexed_struct: <S::Map<&'a mut T> as EnumIndexed<&'a mut T, N>>::Map<Option<&'a mut T>>,
    indexes: Indexes<T, N, S>,
}

impl<'a, T, const N: usize, S: EnumIndexed<T, N>> IterMut<'a, T, N, S> {
    pub fn new(indexed_struct: &'a mut S) -> Self {
        IterMut {
            indexed_struct: indexed_struct.as_mut().map(Some),
            indexes: Indexes::new(),
        }
    }
}

impl<'a, T, const N: usize, S: EnumIndexed<T, N>> Iterator for IterMut<'a, T, N, S> {
    type Item = (S::Enum, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        self.indexes.next_front().and_then(|variant| {
            self.indexed_struct[variant]
                .take()
                .map(|value| (variant, value))
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let line = self.len();
        (line, Some(line))
    }
}

impl<T, const N: usize, S: EnumIndexed<T, N>> DoubleEndedIterator for IterMut<'_, T, N, S> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.indexes.next_back().and_then(|variant| {
            self.indexed_struct[variant]
                .take()
                .map(|value| (variant, value))
        })
    }
}

impl<T, const N: usize, S: EnumIndexed<T, N>> ExactSizeIterator for IterMut<'_, T, N, S> {
    fn len(&self) -> usize {
        self.indexes.remaining_count()
    }
}

impl<T, const N: usize, S: EnumIndexed<T, N>> FusedIterator for IterMut<'_, T, N, S> {}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct IntoIter<K, V, const N: usize> {
    inner: array::IntoIter<(K, V), N>,
}

impl<K, V, const N: usize> IntoIter<K, V, N> {
    pub fn new<S: EnumIndexed<V, N, Enum = K>>(indexed_struct: S) -> Self {
        IntoIter {
            inner: indexed_struct.into_array_enumerated().into_iter(),
        }
    }
}

impl<K, V, const N: usize> Iterator for IntoIter<K, V, N> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<K, V, const N: usize> DoubleEndedIterator for IntoIter<K, V, N> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back()
    }
}

impl<K, V, const N: usize> ExactSizeIterator for IntoIter<K, V, N> {}

impl<K, V, const N: usize> FusedIterator for IntoIter<K, V, N> {}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct Values<'a, T, const N: usize, S: EnumIndexed<T, N>> {
    inner: Iter<'a, T, N, S>,
}

impl<'a, T: 'a, const N: usize, S: EnumIndexed<T, N>> Values<'a, T, N, S> {
    pub(super) fn new(indexed_struct: &'a S) -> Self {
        Values {
            inner: indexed_struct.iter(),
        }
    }
}

impl<'a, T: 'a, const N: usize, S: EnumIndexed<T, N>> Iterator for Values<'a, T, N, S> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(_, value)| value)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, T: 'a, const N: usize, S: EnumIndexed<T, N>> DoubleEndedIterator for Values<'a, T, N, S> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(|(_, value)| value)
    }
}

impl<'a, T: 'a, const N: usize, S: EnumIndexed<T, N>> ExactSizeIterator for Values<'a, T, N, S> {}

impl<'a, T: 'a, const N: usize, S: EnumIndexed<T, N>> FusedIterator for Values<'a, T, N, S> {}

////////////////////////////////////////////////////////////////////////////////

pub struct ValuesMut<'a, T, const N: usize, S: EnumIndexed<T, N>> {
    inner: IterMut<'a, T, N, S>,
}

impl<'a, T: 'a, const N: usize, S: EnumIndexed<T, N>> ValuesMut<'a, T, N, S> {
    pub(super) fn new(indexed_struct: &'a mut S) -> Self {
        ValuesMut {
            inner: indexed_struct.iter_mut(),
        }
    }
}

impl<'a, T: 'a, const N: usize, S: EnumIndexed<T, N>> Iterator for ValuesMut<'a, T, N, S> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(_, value)| value)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, T: 'a, const N: usize, S: EnumIndexed<T, N>> DoubleEndedIterator
    for ValuesMut<'a, T, N, S>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(|(_, value)| value)
    }
}

impl<'a, T: 'a, const N: usize, S: EnumIndexed<T, N>> ExactSizeIterator for ValuesMut<'a, T, N, S> {}

impl<'a, T: 'a, const N: usize, S: EnumIndexed<T, N>> FusedIterator for ValuesMut<'a, T, N, S> {}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
pub struct IntoValues<T, const N: usize> {
    inner: array::IntoIter<T, N>,
}

impl<T, const N: usize> IntoValues<T, N> {
    pub fn new<S: EnumIndexed<T, N>>(entries: S) -> Self {
        IntoValues {
            inner: entries.into_array().into_iter(),
        }
    }
}

impl<T, const N: usize> Iterator for IntoValues<T, N> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<T, const N: usize> DoubleEndedIterator for IntoValues<T, N> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back()
    }
}

impl<T, const N: usize> ExactSizeIterator for IntoValues<T, N> {}

impl<T, const N: usize> FusedIterator for IntoValues<T, N> {}

////////////////////////////////////////////////////////////////////////////////

mod indexes {
    use core::fmt::Debug;
    use core::marker::PhantomData;

    use crate::indexed_struct::EnumIndexed;

    pub(super) struct Indexes<T, const N: usize, S> {
        front: usize,
        back: usize,
        phantom: PhantomData<(T, S)>,
    }

    impl<T, const N: usize, S: EnumIndexed<T, N>> Indexes<T, N, S> {
        pub(super) fn new() -> Self {
            Indexes {
                front: 0,
                back: N,
                phantom: PhantomData,
            }
        }

        pub(super) fn next_front(&mut self) -> Option<S::Enum> {
            (self.front < self.back).then(|| {
                let variant = S::variant_from_index(self.front);
                self.front += 1;
                variant
            })
        }

        pub(super) fn next_back(&mut self) -> Option<S::Enum> {
            (self.front < self.back).then(|| {
                self.back -= 1;
                S::variant_from_index(self.back)
            })
        }

        pub(super) fn remaining_count(self) -> usize {
            self.back - self.front
        }
    }

    // Traits explicitly implement to not depend on the `S` bounds.

    impl<T, const N: usize, S> Debug for Indexes<T, N, S> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("Indexes")
                .field("front", &self.front)
                .field("back", &self.back)
                .finish()
        }
    }

    impl<T, const N: usize, S> Clone for Indexes<T, N, S> {
        fn clone(&self) -> Self {
            *self
        }
    }

    impl<T, const N: usize, S> Copy for Indexes<T, N, S> {}

    impl<T, const N: usize, S> PartialEq for Indexes<T, N, S> {
        fn eq(&self, other: &Self) -> bool {
            self.front == other.front && self.back == other.back
        }
    }

    impl<T, const N: usize, S> Eq for Indexes<T, N, S> {}
}
