mod refs {
    use crate::{Indexed, IndexedOwned, IndexedRef, Indices, Len};

    impl<A> Len for &A
    where
        A: Len + ?Sized,
    {
        fn len(&self) -> usize {
            (**self).len()
        }
    }

    impl<'a, A, Idx> Indices<'a, Idx> for &A
    where
        A: Indices<'a, Idx> + ?Sized,
    {
        type Indices = A::Indices;

        fn indices(&'a self) -> Self::Indices {
            (**self).indices()
        }
    }

    impl<'a, A, Idx> Indexed<'a, Idx> for &A
    where
        A: Indexed<'a, Idx> + ?Sized,
    {
        type Output = A::Output;

        fn get(&'a self, index: Idx) -> Option<Self::Output> {
            (**self).get(index)
        }
    }

    impl<A, Idx> IndexedOwned<Idx> for &A
    where
        A: IndexedOwned<Idx> + ?Sized,
    {
        type Output = A::Output;

        fn get(&self, index: Idx) -> Option<Self::Output> {
            (**self).get(index)
        }
    }

    impl<A, Idx> IndexedRef<Idx> for &A
    where
        A: IndexedRef<Idx> + ?Sized,
    {
        type Target = A::Target;

        fn get(&self, index: Idx) -> Option<&Self::Target> {
            (**self).get(index)
        }
    }
}

mod muts {
    use crate::{IndexedMut, IndexedRef, Indices, Len};

    impl<A> Len for &mut A
    where
        A: Len + ?Sized,
    {
        fn len(&self) -> usize {
            (**self).len()
        }
    }

    impl<'a, A, Idx> Indices<'a, Idx> for &mut A
    where
        A: Indices<'a, Idx> + ?Sized,
    {
        type Indices = A::Indices;

        fn indices(&'a self) -> Self::Indices {
            (**self).indices()
        }
    }

    impl<A, Idx> IndexedRef<Idx> for &mut A
    where
        A: IndexedRef<Idx> + ?Sized,
    {
        type Target = A::Target;

        fn get(&self, index: Idx) -> Option<&Self::Target> {
            (**self).get(index)
        }
    }

    impl<A, Idx> IndexedMut<Idx> for &mut A
    where
        A: IndexedMut<Idx> + ?Sized,
    {
        fn get_mut(&mut self, index: Idx) -> Option<&mut Self::Target> {
            (**self).get_mut(index)
        }
    }
}

mod slice {
    use core::ops::Range;

    use crate::{IndexedMut, IndexedRef, Indices, Len};

    impl<T> Len for [T] {
        fn len(&self) -> usize {
            self.len()
        }
    }

    /*
        NOTE: when the `SliceIndex` methods stabilize, we may have a generic `impl<'a, T, Idx> Indices<'a, Idx>
        for [T]` instead of only `impl<'a, T> Indices<'a, usize> for [T]`.
        The same applies for `[T; N]` and `Vec<T>`, and with the `IndexedRef` and `IndexedMut` traits.
    */

    impl<'a, T> Indices<'a, usize> for [T] {
        type Indices = Range<usize>;

        fn indices(&'a self) -> Self::Indices {
            0..self.len()
        }
    }

    impl<T> IndexedRef<usize> for [T] {
        type Target = T;

        fn get(&self, index: usize) -> Option<&Self::Target> {
            self.get(index)
        }
    }

    impl<T> IndexedMut<usize> for [T] {
        fn get_mut(&mut self, index: usize) -> Option<&mut Self::Target> {
            self.get_mut(index)
        }
    }
}

mod array {
    use core::ops::Range;

    use crate::{IndexedMut, IndexedRef, Indices, Len};

    impl<T, const N: usize> Len for [T; N] {
        fn len(&self) -> usize {
            self.as_slice().len()
        }
    }

    impl<'a, T, const N: usize> Indices<'a, usize> for [T; N] {
        type Indices = Range<usize>;

        fn indices(&'a self) -> Self::Indices {
            0..self.len()
        }
    }

    impl<T, const N: usize> IndexedRef<usize> for [T; N] {
        type Target = T;

        fn get(&self, index: usize) -> Option<&Self::Target> {
            self.as_slice().get(index)
        }
    }

    impl<T, const N: usize> IndexedMut<usize> for [T; N] {
        fn get_mut(&mut self, index: usize) -> Option<&mut Self::Target> {
            self.as_mut_slice().get_mut(index)
        }
    }
}

#[cfg(feature = "alloc")]
mod vec {
    use alloc::vec::Vec;
    use core::ops::Range;

    use crate::{IndexedMut, IndexedRef, Indices, Len};

    impl<T> Len for Vec<T> {
        fn len(&self) -> usize {
            self.as_slice().len()
        }
    }

    impl<'a, T> Indices<'a, usize> for Vec<T> {
        type Indices = Range<usize>;

        fn indices(&'a self) -> Self::Indices {
            0..self.len()
        }
    }

    impl<T> IndexedRef<usize> for Vec<T> {
        type Target = T;

        fn get(&self, index: usize) -> Option<&Self::Target> {
            self.as_slice().get(index)
        }
    }

    impl<T> IndexedMut<usize> for Vec<T> {
        fn get_mut(&mut self, index: usize) -> Option<&mut Self::Target> {
            self.as_mut_slice().get_mut(index)
        }
    }
}

#[cfg(feature = "alloc")]
mod vec_deque {
    use alloc::collections::VecDeque;
    use core::ops::Range;

    use crate::{IndexedMut, IndexedRef, Indices, Len};

    impl<T> Len for VecDeque<T> {
        fn len(&self) -> usize {
            self.len()
        }
    }

    impl<'a, T> Indices<'a, usize> for VecDeque<T> {
        type Indices = Range<usize>;

        fn indices(&'a self) -> Self::Indices {
            0..self.len()
        }
    }

    impl<T> IndexedRef<usize> for VecDeque<T> {
        type Target = T;

        fn get(&self, index: usize) -> Option<&Self::Target> {
            self.get(index)
        }
    }

    impl<T> IndexedMut<usize> for VecDeque<T> {
        fn get_mut(&mut self, index: usize) -> Option<&mut Self::Target> {
            self.get_mut(index)
        }
    }
}

#[cfg(feature = "alloc")]
mod btree_map {
    use alloc::collections::{BTreeMap, btree_map};
    use core::borrow::Borrow;
    use core::iter::Map;

    use crate::{IndexedMut, IndexedRef, Indices, Len};

    impl<K, V> Len for BTreeMap<K, V> {
        fn len(&self) -> usize {
            self.len()
        }
    }

    impl<'a, K: 'a, Q, V: 'a> Indices<'a, &'a Q> for BTreeMap<K, V>
    where
        K: Borrow<Q>,
        Q: ?Sized,
    {
        type Indices = Map<btree_map::Keys<'a, K, V>, fn(&K) -> &Q>;

        fn indices(&'a self) -> Self::Indices {
            self.keys().map(Borrow::borrow)
        }
    }

    impl<K, Q, V> IndexedRef<&Q> for BTreeMap<K, V>
    where
        K: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
        type Target = V;

        fn get(&self, index: &Q) -> Option<&Self::Target> {
            self.get(index)
        }
    }

    impl<K, Q, V> IndexedMut<&Q> for BTreeMap<K, V>
    where
        K: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
        fn get_mut(&mut self, index: &Q) -> Option<&mut Self::Target> {
            self.get_mut(index)
        }
    }
}

#[cfg(feature = "std")]
mod hash_map {
    use core::borrow::Borrow;
    use core::hash::{BuildHasher, Hash};
    use core::iter::Map;
    use std::collections::{HashMap, hash_map};

    use crate::{IndexedMut, IndexedRef, Indices, Len};

    impl<K, V, S> Len for HashMap<K, V, S> {
        fn len(&self) -> usize {
            self.len()
        }
    }

    impl<'a, K: 'a, Q, V: 'a, S> Indices<'a, &'a Q> for HashMap<K, V, S>
    where
        K: Borrow<Q>,
    {
        type Indices = Map<hash_map::Keys<'a, K, V>, fn(&K) -> &Q>;

        fn indices(&'a self) -> Self::Indices {
            self.keys().map(Borrow::borrow)
        }
    }

    impl<K, Q, V, S> IndexedRef<&Q> for HashMap<K, V, S>
    where
        K: Borrow<Q> + Eq + Hash,
        Q: Eq + Hash,
        S: BuildHasher,
    {
        type Target = V;

        fn get(&self, index: &Q) -> Option<&Self::Target> {
            self.get(index)
        }
    }

    impl<K, Q, V, S> IndexedMut<&Q> for HashMap<K, V, S>
    where
        K: Borrow<Q> + Eq + Hash,
        Q: Eq + Hash,
        S: BuildHasher,
    {
        fn get_mut(&mut self, index: &Q) -> Option<&mut Self::Target> {
            self.get_mut(index)
        }
    }
}
