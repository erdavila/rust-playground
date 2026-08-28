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

mod slice {
    use core::ops::Range;

    use crate::{Indexed, Indices, Len};

    impl<T> Len for [T] {
        fn len(&self) -> usize {
            self.len()
        }
    }

    /*
        NOTE: when the `SliceIndex` methods stabilize, we may have a generic `impl<'a, T, Idx> Indices<'a, Idx>
        for [T]` instead of only `impl<'a, T> Indices<'a, usize> for [T]`.
        The same applies for `[T; N]` and `Vec<T>`, and with the `Indexed` trait.
    */

    impl<'a, T> Indices<'a, usize> for [T] {
        type Indices = Range<usize>;

        fn indices(&'a self) -> Self::Indices {
            0..self.len()
        }
    }

    impl<'a, T: 'a> Indexed<'a, usize> for [T] {
        type Output = &'a T;

        fn get(&'a self, index: usize) -> Option<Self::Output> {
            self.get(index)
        }
    }
}

mod array {
    use core::ops::Range;

    use crate::{Indexed, Indices, Len};

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

    impl<'a, T: 'a, const N: usize> Indexed<'a, usize> for [T; N] {
        type Output = &'a T;

        fn get(&'a self, index: usize) -> Option<Self::Output> {
            self.as_slice().get(index)
        }
    }
}

#[cfg(feature = "alloc")]
mod vec {
    use alloc::vec::Vec;
    use core::ops::Range;

    use crate::{Indexed, Indices, Len};

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

    impl<'a, T: 'a> Indexed<'a, usize> for Vec<T> {
        type Output = &'a T;

        fn get(&'a self, index: usize) -> Option<Self::Output> {
            self.as_slice().get(index)
        }
    }
}

#[cfg(feature = "alloc")]
mod vec_deque {
    use alloc::collections::VecDeque;
    use core::ops::Range;

    use crate::{Indexed, Indices, Len};

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

    impl<'a, T: 'a> Indexed<'a, usize> for VecDeque<T> {
        type Output = &'a T;

        fn get(&'a self, index: usize) -> Option<Self::Output> {
            self.get(index)
        }
    }
}

#[cfg(feature = "alloc")]
mod btree_map {
    use alloc::collections::{BTreeMap, btree_map};
    use core::borrow::Borrow;
    use core::iter::Map;

    use crate::{Indexed, Indices, Len};

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

    impl<'a, K: 'a, Q, V: 'a> Indexed<'a, &'a Q> for BTreeMap<K, V>
    where
        K: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
        type Output = &'a V;

        fn get(&'a self, index: &Q) -> Option<Self::Output> {
            self.get(index)
        }
    }
}

#[cfg(feature = "std")]
mod hash_map {
    use core::borrow::Borrow;
    use core::hash::{BuildHasher, Hash};
    use core::iter::Map;
    use std::collections::{HashMap, hash_map};

    use crate::{Indexed, Indices, Len};

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

    impl<'a, K: 'a, Q, V: 'a, S> Indexed<'a, &'a Q> for HashMap<K, V, S>
    where
        K: Borrow<Q> + Eq + Hash,
        Q: Eq + Hash,
        S: BuildHasher,
    {
        type Output = &'a V;

        fn get(&'a self, index: &Q) -> Option<Self::Output> {
            self.get(index)
        }
    }
}
