use core::marker::PhantomData;

use crate::interval::Interval;

pub struct IntoIter<K, V> {
    phantom: PhantomData<(K, V)>,
}
impl<K, V> Iterator for IntoIter<K, V> {
    type Item = (Interval<K>, V);

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

pub struct PreviousEntries<'a, K, V> {
    inner: alloc::vec::IntoIter<<Self as Iterator>::Item>,
    phantom: PhantomData<(&'a K, &'a V)>,
}

impl<'a, K, V> PreviousEntries<'a, K, V> {
    pub(crate) fn new(inner: alloc::vec::IntoIter<<Self as Iterator>::Item>) -> Self {
        PreviousEntries {
            inner,
            phantom: PhantomData,
        }
    }
}

impl<K, V> Iterator for PreviousEntries<'_, K, V> {
    type Item = (Interval<K>, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

pub struct Iter<'a, K, V> {
    phantom: PhantomData<(&'a K, &'a V)>,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a Interval<K>, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

pub struct IterMut<'a, K, V> {
    phantom: PhantomData<(&'a K, &'a mut V)>,
}

impl<'a, K, V> Iterator for IterMut<'a, K, V> {
    type Item = (&'a Interval<K>, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

pub struct IterContaining<'a, K, V> {
    phantom: PhantomData<(&'a K, &'a V)>,
}

impl<'a, K, V> Iterator for IterContaining<'a, K, V> {
    type Item = (&'a Interval<K>, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

pub struct IterContainingMut<'a, K, V> {
    phantom: PhantomData<(&'a K, &'a V)>,
}

impl<'a, K, V> Iterator for IterContainingMut<'a, K, V> {
    type Item = (&'a Interval<K>, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
