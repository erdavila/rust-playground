use std::marker::PhantomData;
use std::ops::{Index, IndexMut};

pub(crate) trait EnumMapKey<const N: usize>: Copy + Eq {
    fn all() -> [Self; N];

    fn to_usize(self) -> usize {
        Self::all().into_iter().position(|k| k == self).unwrap()
    }
}

pub(crate) struct EnumMap<K, V, const N: usize> {
    array: [V; N],
    phantom: PhantomData<K>,
}
impl<K: EnumMapKey<N>, V, const N: usize> EnumMap<K, V, N> {
    pub(crate) fn from_fn(f: impl FnMut(K) -> V) -> Self {
        let array = K::all().map(f);
        EnumMap {
            array,
            phantom: PhantomData,
        }
    }
}
impl<K: EnumMapKey<N>, V, const N: usize> Index<K> for EnumMap<K, V, N> {
    type Output = V;

    fn index(&self, index: K) -> &Self::Output {
        &self.array[index.to_usize()]
    }
}
impl<K: EnumMapKey<N>, V, const N: usize> IndexMut<K> for EnumMap<K, V, N> {
    fn index_mut(&mut self, index: K) -> &mut Self::Output {
        &mut self.array[index.to_usize()]
    }
}
