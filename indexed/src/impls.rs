mod refs {
    use crate::Indexed;

    impl<'a, A, Idx> Indexed<'a, Idx> for &A
    where
        A: Indexed<'a, Idx> + ?Sized,
    {
        type Output = A::Output;

        type Indices = A::Indices;

        fn get(&'a self, index: Idx) -> Option<Self::Output> {
            (**self).get(index)
        }

        fn indices(&'a self) -> Self::Indices {
            (**self).indices()
        }

        fn len(&self) -> usize {
            (**self).len()
        }
    }
}

mod slice {
    use core::ops::Range;

    use crate::Indexed;

    /*
        NOTE: when the `SliceIndex` methods stabilize, we may have a generic `impl<T, Idx> Indexed<Idx>
        for [T]` instead of only `impl<T> Indexed<usize> for [T]`.
        The same applies for `[T; N]` and `Vec<T>`.
    */
    impl<'a, T: 'a> Indexed<'a, usize> for [T] {
        type Output = &'a T;

        type Indices = Range<usize>;

        fn get(&'a self, index: usize) -> Option<Self::Output> {
            self.get(index)
        }

        fn indices(&self) -> Self::Indices {
            0..self.len()
        }

        fn len(&self) -> usize {
            self.len()
        }
    }
}

mod array {
    use core::ops::Range;

    use crate::Indexed;

    impl<'a, T: 'a, const N: usize> Indexed<'a, usize> for [T; N] {
        type Output = &'a T;

        type Indices = Range<usize>;

        fn get(&'a self, index: usize) -> Option<Self::Output> {
            self.as_slice().get(index)
        }

        fn indices(&self) -> Self::Indices {
            0..self.len()
        }

        fn len(&self) -> usize {
            self.as_slice().len()
        }
    }
}

#[cfg(feature = "alloc")]
mod vec {
    use alloc::vec::Vec;
    use core::ops::Range;

    use crate::Indexed;

    impl<'a, T: 'a> Indexed<'a, usize> for Vec<T> {
        type Output = &'a T;

        type Indices = Range<usize>;

        fn get(&'a self, index: usize) -> Option<Self::Output> {
            self.as_slice().get(index)
        }

        fn indices(&self) -> Self::Indices {
            0..self.len()
        }

        fn len(&self) -> usize {
            self.as_slice().len()
        }
    }
}

#[cfg(feature = "alloc")]
mod vec_deque {
    use alloc::collections::VecDeque;
    use core::ops::Range;

    use crate::Indexed;

    impl<'a, T: 'a> Indexed<'a, usize> for VecDeque<T> {
        type Output = &'a T;

        type Indices = Range<usize>;

        fn get(&'a self, index: usize) -> Option<Self::Output> {
            self.get(index)
        }

        fn indices(&self) -> Self::Indices {
            0..self.len()
        }

        fn len(&self) -> usize {
            self.len()
        }
    }
}

#[cfg(feature = "alloc")]
mod btree_map {
    use alloc::collections::{BTreeMap, btree_map};
    use core::borrow::Borrow;
    use core::iter::Map;

    use crate::Indexed;

    impl<'a, K: 'a, Q, V: 'a> Indexed<'a, &'a Q> for BTreeMap<K, V>
    where
        K: Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
        type Output = &'a V;

        type Indices = Map<btree_map::Keys<'a, K, V>, fn(&K) -> &Q>;

        fn get(&'a self, index: &Q) -> Option<Self::Output> {
            self.get(index)
        }

        fn indices(&'a self) -> Self::Indices {
            self.keys().map(Borrow::borrow)
        }

        fn len(&self) -> usize {
            self.len()
        }
    }
}

#[cfg(feature = "std")]
mod hash_map {
    use core::borrow::Borrow;
    use core::hash::{BuildHasher, Hash};
    use core::iter::Map;
    use std::collections::{HashMap, hash_map};

    use crate::Indexed;

    impl<'a, K: 'a, Q, V: 'a, S> Indexed<'a, &'a Q> for HashMap<K, V, S>
    where
        K: Borrow<Q> + Eq + Hash,
        Q: Eq + Hash,
        S: BuildHasher,
    {
        type Output = &'a V;

        type Indices = Map<hash_map::Keys<'a, K, V>, fn(&K) -> &Q>;

        fn get(&'a self, index: &Q) -> Option<Self::Output> {
            self.get(index)
        }

        fn indices(&'a self) -> Self::Indices {
            self.keys().map(Borrow::borrow)
        }

        fn len(&self) -> usize {
            self.len()
        }
    }
}
